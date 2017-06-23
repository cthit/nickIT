from flask import Flask, Response

app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World!'

@app.route('/<id>')
def example(id=None):
    resp = Response(id)
    resp.headers['Access-Control-Allow-Origin'] = 'http://localhost:3000'
    return resp
