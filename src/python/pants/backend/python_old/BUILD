# Copyright 2014 Pants project contributors (see CONTRIBUTORS.md).
# Licensed under the Apache License, Version 2.0 (see LICENSE).

python_library(
  name = 'plugin',
  sources = ['__init__.py', 'register.py'],
  dependencies = [
    'src/python/pants/backend/python:plugin',
    'src/python/pants/backend/python/tasks:python',
    'src/python/pants/backend/python/tasks2',
    'src/python/pants/base:deprecated',
    'src/python/pants/goal:task_registrar',
  ]
)
