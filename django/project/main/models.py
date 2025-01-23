from django.db import models

# Create your models here.
class User(models.Model):
    """
    User model that represents a user in the system.
    """
    user = models.CharField(max_length=100)