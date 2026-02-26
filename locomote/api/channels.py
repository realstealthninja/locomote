from typing import Any, override
from channels.generic.websocket import JsonWebsocketConsumer


class RegistratorConsumer(JsonWebsocketConsumer):
    @override
    def connect(self) -> None:
        return self.accept()

    @override
    def disconnect(self, code: int) -> None:
        pass

    @override
    def receive_json(self, content: Any, **kwargs: Any) -> None:
        # {'cardid': 'adsfasdfasdf'}
        # {'command': 'register', 'data': {}
        # {'command': }
