#!/usr/bin/env python

import flask
from sqlalchemy import text
from flask import request
from flask_sqlalchemy import SQLAlchemy

app = flask.Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite+pysqlite:///commenter_python.db'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
db = SQLAlchemy(app)

class Comment(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    author = db.Column(db.String(80), nullable=False)
    text = db.Column(db.Text, nullable=False)

@app.post('/submit')
def submit():
    try:
        auth = request.json['author']
        body = request.json['text']
        comm = Comment(author=auth, text=body)
        db.session.add(comm)
        db.session.commit()
        return '', 200
    except KeyError:
        return '', 400

@app.route('/')
def index():
    comments = Comment.query.all()
    return flask.render_template('index.html', commentcount=len(comments), comments=comments)

def get_app():
    return app

if __name__ == '__main__':
    app.run(debug=True)