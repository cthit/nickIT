from flask import Flask, Response, request, jsonify, abort, render_template
from ldap3 import Server, Connection, ALL
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
    rv = jsonify(nick_list=parse_query(query))
    response = Flask.make_response(app, rv)
    response.headers['Access-Control-Allow-Origin'] = '*'
    return response

@app.route('/search/')
def handle_empty_search():
    abort(400)

@app.errorhandler(400)
def bad_request(error):
    response = Flask.make_response(app, 'bad request')
    response.headers['Access-Control-Allow-Origin'] = '*'
    response.status = '400'
    return response
