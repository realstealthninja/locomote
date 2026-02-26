from django.db import models

from django.contrib.auth.models import User
from django.db.models.signals import post_save
from django.dispatch import receiver




class Scanner(models.Model):
    location = models.CharField(max_length=200)
    cost = models.FloatField(null=True)


class Card(models.Model):
    user = models.ForeignKey(User, related_name="cards", on_delete=models.CASCADE)

    key_a = models.CharField(max_length=4)
    card_id = models.CharField(max_length=7)

    balance = models.FloatField()

    def of_user(self, user: User):
        return self.user == user


class Ticket(models.Model):
    user = models.ForeignKey(User, related_name="tickets", on_delete=models.CASCADE)
    card = models.ForeignKey(Card, related_name="tickets", on_delete=models.CASCADE)

    validity = models.DateTimeField()
    duration = models.DurationField()
    origin = models.CharField(max_length=200)

    destination = models.CharField(max_length=200)


@receiver(post_save, sender=User)
def create_or_update_user_profile(sender, instance, created, **kwargs):
    if created:
        UserProfile.objects.create()
    else:
        instance.userprofile.save()
