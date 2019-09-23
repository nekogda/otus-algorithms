# HW18 otus-algorithms

Homework contains implementations of dijkstra (heap based) path searching algorithm.

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/18-dijkstra/dijkstra-py
$ pip3 install pipenv
$ pipenv install
```

#### run quick tests
```
$ pipenv run pytest -v
```
#### coverage report
```
$ pipenv run pytest --cov=dijkstra --cov-report=html && firefox htmlcov/index.html
```
