from django.contrib.auth.models import User
from django.shortcuts import render
from rest_framework.response import Response
from rest_framework import status, viewsets, response
from rest_framework.views import APIView
from rest_framework import permissions

from .models import Card

from .serializers import CardSerializer, UserRegistrationSerializer


# Create your views here.
class UserRegistration(viewsets.ModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserRegistrationSerializer


class CardList(APIView):
    permission_classes = [permissions.IsAuthenticated]

    def get(self, request, format=None):
        user = request.user
        cards = Card.objects.filter(user=user)
        serializer = CardSerializer(cards, many=True)
        return response.Response(serializer.data)

    def post(self, request, format=None):
        serializer = CardSerializer(data=request.data)
        if serializer.is_valid():
            serializer.save()
            return Response(serializer.data, status=status.HTTP_201_CREATED)
        else:
            return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)
