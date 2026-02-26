from django.urls import path
from .views import CardList, UserRegistration


urlpatterns = [
    path(
        "user/register",
        UserRegistration.as_view,
    ),
    path("cards/", CardList.as_view()),
]
