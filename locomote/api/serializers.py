from dataclasses import field
from typing import override
from django.contrib.auth.models import User
from rest_framework import serializers

from .models import Card, Ticket


class UserSerializer(serializers.ModelSerializer):
    cards = serializers.PrimaryKeyRelatedField(many=True, queryset=Card.objects.all())
    tickets = serializers.PrimaryKeyRelatedField(
        many=True, queryset=Ticket.objects.all()
    )

    class Meta:
        model = User
        fields = ["id", "username", "cards"]


class CardSerializer(serializers.ModelSerializer):
    tickets = serializers.PrimaryKeyRelatedField(
        many=True, queryset=Ticket.objects.all()
    )

    @override
    def create(self, validated_data):
        return Card.objects.create(**validated_data)

    class Meta:
        model = Card
        field = "__all__"


class UserRegistrationSerializer(serializers.ModelSerializer):
    password2 = serializers.CharField(style={"input_type": "password"}, write_only=True)

    class Meta:
        model = User
        fields = ["username", "email", "password"]
        extra_kwargs = {"password": {"write_only": True}}

    @override
    def create(self, validated_data):
        user = User.objects.create_user(
            username=validated_data["username"],
            email=validated_data["email"],
            password=validated_data["password"],
        )
        return user
