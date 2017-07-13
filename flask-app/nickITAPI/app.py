from flask import Flask, Response, request, jsonify
import json
import re

app = Flask(__name__) # create the application instance :)

@app.route('/')
def hello_world():
    return 'Hello, World!'

def parse_query(query):
    nick_list = re.compile('\s*[,]+\s*').split(query)
    nick_list = list(filter(None, nick_list))
    return nick_list

@app.route('/search/<query>')
def handle_search(query=None):
    response = Flask.make_response(app, jsonify(nick_list=parse_query(query)))
    response.headers['Access-Control-Allow-Origin'] = '*'
    return response
