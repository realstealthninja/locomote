from django.contrib import admin

from .models import Card, Scanner, Ticket

# Register your models here.
admin.site.register(Card)
admin.site.register(Ticket)
admin.site.register(Scanner)
