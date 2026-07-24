from peewee import ForeignKeyField
import json
import os
from peewee import SqliteDatabase,Model,CharField,BlobField,TextField,IntegerField,FloatField,CompositeKey

class DBManager:
    def __init__(self, db_path = "db.db"):
        self.db = SqliteDatabase(db_path)

    class User(Model):
        username = CharField(unique=True)
        password = CharField()
        