from typing import Tuple


def _parse_key(key: bytes) -> Tuple[int, int]:
    x, n = [
        int(seq) for seq in key.split(b':')
    ]
    return (x, n)


def encrypt(buf: bytes, key: bytes) -> bytes:
    e, n = _parse_key(key)
    result = bytearray()
    for i in range(len(buf)):
        x = int.from_bytes(buf[i:i+1], byteorder='little', signed=False)
        result.extend(((x ** e) % n).to_bytes(
            8, byteorder='little', signed=False)
        )
    return result


def decrypt(buf: bytes, key: bytes) -> bytes:
    d, n = _parse_key(key)
    result = bytearray()
    for i in range(0, len(buf), 8):
        x = int.from_bytes(buf[i:i+8], byteorder='little', signed=False)
        result.append((x ** d) % n)
    return result
