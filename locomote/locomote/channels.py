from typing import Any, override
from channels.generic.websocket import AsyncJsonWebsocketConsumer
import json


class ResigratorConsumer(AsyncJsonWebsocketConsumer):
    @override
    async def connect(self):
        await self.accept()

    @override
    async def disconnect(self, code: int) -> None:
        return await super().disconnect(code)

    @override
    async def receive(
        self,
        text_data: str | None = None,
        bytes_data: bytes | None = None,
        **kwargs: Any,
    ) -> None:
        print(text_data)
