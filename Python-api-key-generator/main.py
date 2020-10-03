"""
Created by Epic at 10/01/20
Updated by Arthurdw
"""
from flask import Flask, request
from waitress import serve
from random import choice
from json import dumps

app = Flask(__name__)

CHARSET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~."


@app.route("/api/generator/key")
def generate_key():
    return dumps({"key": ''.join([choice(CHARSET) for _ in range(int(request.args['length']))])})


serve(app, port=8000)
