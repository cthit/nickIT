from flask import Flask, Response

app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World!'

@app.route('/search/<query>')
def example(query=None):
    resp = Response(id)
    resp.headers['Access-Control-Allow-Origin'] = 'http://localhost:3000'
    return resp
