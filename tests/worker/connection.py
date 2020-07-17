import asyncio
import random
from asyncio import StreamReader

from cloudpickle import dumps, loads

import pytest

from python.rsds.subworker.conn import connect_to_unix_socket, read_message, write_message
from python.rsds.subworker.subworker import Subworker


async def unix_server():
    connect_fut = asyncio.Future()

    async def cb(reader, writer):
        connect_fut.set_result((reader, writer))

    path = f"/tmp/rsds-{random.randint(0, 1024)}"
    srv = await asyncio.start_unix_server(cb, path)
    return srv, path, connect_fut


async def create_subworker(id: int = 0):
    server, path, connect_fut = await unix_server()
    socket = await connect_to_unix_socket(path)
    reader, writer = await connect_fut
    subworker = Subworker(id, socket)
    await init_subworker(subworker, reader)
    return subworker, reader, writer


@pytest.mark.asyncio
async def test_send_receive():
    server, path, connect_fut = await unix_server()
    socket = await connect_to_unix_socket(path)
    reader, writer = await connect_fut

    message = b"Hello world"
    await socket.send_message(message)
    assert await read_message(reader) == message


async def init_subworker(subworker: Subworker, reader: StreamReader):
    asyncio.get_event_loop().create_task(subworker.run())
    response = await read_message(reader)
    assert response == {"subworker_id": subworker.subworker_id}


@pytest.mark.asyncio
async def test_compute_success():
    subworker, reader, writer = await create_subworker()

    key = "key"
    await write_message({
        "op": "ComputeTask",
        "key": key,
        "function": dumps(lambda x: x + 1),
        "args": dumps([1])
    }, writer)

    response = await read_message(reader)
    assert response["op"] == "TaskFinished"
    assert response["key"] == key
    assert loads(response["result"]) == 2


@pytest.mark.asyncio
async def test_compute_error():
    subworker, reader, writer = await create_subworker()

    def func():
        raise Exception("foo")

    key = "key"
    await write_message({
        "op": "ComputeTask",
        "key": key,
        "function": dumps(func),
        "args": dumps(())
    }, writer)

    response = await read_message(reader)
    assert response["op"] == "TaskErrored"
    assert response["key"] == key
    assert str(loads(response["error"])) == "foo"
