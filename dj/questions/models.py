from django.db import models

# Create your models here.

class Question(models.Model):
    user_name = models.CharField(max_length=100, help_text = "name of user who is posting the question")
    content = models.TextField(help_text = "content of the question being asked from streamer")
    vote_count = models.IntegerField(default=0)
    host = models.CharField(max_length=31, help_text = "user ip address, who posted this question as identity")
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "questions"
