import pytest
from ciphers import xor, xor_cbc, rsa


dataset = [
    ('hello world!', 'x'),
    ('hello world!', '42'),
    ('hello world!', 'xyz'),
    ('hello', 'world!!'),
]


@pytest.mark.parametrize('case', ((text, key) for text, key in dataset))
def test_xor(case):
    text, key = case
    text = text.encode('utf-8')
    key = key.encode('utf-8')
    encrypted = xor.encrypt(text, key)
    decrypted = xor.decrypt(encrypted, key)
    assert text == decrypted


@pytest.mark.parametrize(
    'case',
    (
        (text, key, block_size)
        for text, key in dataset
        for block_size in [1, 2, 3, 4, 8]
    )
)
def test_xor_cbc(case):
    text, key, block_size = case
    text = text.encode('utf-8')
    key = key.encode('utf-8')
    encrypted = xor_cbc.encrypt(text, key, block_size)
    decrypted = xor_cbc.decrypt(encrypted, key, block_size)
    assert text == decrypted


def test_rsa():
    text = "hello world!".encode('utf-8')
    encrypted = rsa.encrypt(text, b'17:3233')
    decrypted = rsa.decrypt(encrypted, b'413:3233')
    assert text == decrypted
