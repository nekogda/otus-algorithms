# HW24 otus-algorithms

Homework contains implementations of XOR, XOR_CBC, RSA (without key generation) encryption algorithms.

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/24-encryption/encryption-py
$ pip3 install pipenv
$ pipenv install
```

#### run tests
```
$ pipenv run pytest -v
```

#### cli mode
```
$ pipenv run python -m ciphers --help
```

#### cli xor/xor_cbc encrypt/decrypt
```
$ pipenv run python -m ciphers -a enc -t xor -s plain_text.txt -d encrypted_text.txt -k key.txt
$ pipenv run python -m ciphers -a dec -t xor -s encrypted_text.txt -d plain_text.txt -k key.txt
```

#### cli rsa encrypt/decrypt

##### prepare key-pair
```
$ echo -n '17:3233' > public.key
$ echo -n '413:3233' > private.key
```

##### encrypt/decrypt message
```
$ pipenv run python -m ciphers -a enc -t rsa -s plain_text.txt -d encrypted_text.txt -k public.key
$ pipenv run python -m ciphers -a dec -t rsa -s encrypted_text.txt -d plain_text.txt -k private.key
```
