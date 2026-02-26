from django.urls import path

from . import channels

websocket_urlpatterns = [path("ws/registrar", channels.RegistratorConsumer.as_asgi())]
