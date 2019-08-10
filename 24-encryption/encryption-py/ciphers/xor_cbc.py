import random


def encrypt(buf: bytes, key: bytes, block_size: int = 8) -> bytes:
    assert block_size > 0 and len(key) > 0 and len(buf) > 0
    result = bytearray()
    # declare and fill initialisation vector
    iv = bytearray(random.getrandbits(8) for i in range(block_size))
    result.extend(iv)
    enc_block = iv
    for i in range(len(buf)):
        j = i % block_size
        enc_block[j] = enc_block[j] ^ buf[i] ^ key[i % len(key)]
        result.append(enc_block[j])
    return bytes(result)


def decrypt(buf: bytes, key: bytes, block_size: int = 8) -> bytes:
    assert block_size > 0 and len(key) > 0 and len(buf) > 0
    prev_enc_block = bytearray(buf[:block_size])
    result = bytearray()
    open_block = bytearray(block_size)
    dec_block = bytearray(block_size)
    for i in range(block_size, len(buf)):
        j = i % block_size
        open_block[j] = buf[i] ^ key[(i-block_size) % len(key)]
        dec_block[j] = open_block[j] ^ prev_enc_block[j]
        prev_enc_block[j] = buf[i]
        result.append(dec_block[j])
    return bytes(result)
