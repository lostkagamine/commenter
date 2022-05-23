#!/usr/bin/env python

import waitress
from main import get_app
from os import cpu_count

thread_count = int(cpu_count() * 1.25)
print(f'Beginning to serve now on localhost:5004, with {thread_count} Waitress threads.')
waitress.serve(get_app(), listen='localhost:5004', threads=thread_count)