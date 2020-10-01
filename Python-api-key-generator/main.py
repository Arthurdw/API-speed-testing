"""
Created by Epic at 10/1/20
"""
from flask import Flask, request
from waitress import serve
from random import choice
from string import ascii_letters

app = Flask(__name__)


@app.route("/api/generator/key")
def generate_key():
    return "".join([choice(ascii_letters) for i in range(int(request.args["length"]))])


serve(app, port=5050)
