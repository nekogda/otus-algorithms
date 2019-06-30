# HW09 otus-algorithms

Homework contains implementations of avltree.

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/09-avltree
$ pip3 install pipenv
$ pipenv install
```

#### run quick tests
```
$ pipenv run pytest -v
```
#### coverage report
```
$ pipenv run pytest --cov=avltree --cov-report=html && firefox htmlcov/index.html
```
