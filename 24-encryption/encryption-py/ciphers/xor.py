

def encrypt(buf: bytes, key: bytes) -> bytes:
    result = bytearray()
    for i in range(len(buf)):
        result.append(buf[i] ^ key[i % len(key)])
    return bytes(result)


def decrypt(buf: bytes, key: bytes) -> bytes:
    return encrypt(buf, key)
