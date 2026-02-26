from typing import override
from channels.generic.websocket import AsyncJsonWebsocketConsumer


class RegistratorConsumer(AsyncJsonWebsocketConsumer):
    @override
    async def connect(self) -> None:
        return await self.accept()

    @override
    async def disconnect(self, code: int) -> None:
        pass

    @override
    async def receive(
        self, text_data: str | None = None, bytes_data: bytes | None = None
    ) -> None:
        print(text_data)
        print(bytes_data)
