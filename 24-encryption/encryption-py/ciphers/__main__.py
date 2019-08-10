import argparse
import sys
from ciphers import xor
from ciphers import xor_cbc
from ciphers import rsa


def _main():
    parser = argparse.ArgumentParser(prog='ciphers',
                                     description='Encrypt or decrypt files.')
    parser.add_argument('-a', '--action', nargs=1, choices=['enc', 'dec'],
                        required=True)
    parser.add_argument('-s', '--src', default=sys.stdin.buffer, nargs='?',
                        type=argparse.FileType('rb'))
    parser.add_argument('-d', '--dst', default=sys.stdout.buffer, nargs='?',
                        type=argparse.FileType('wb'))
    parser.add_argument('-k', '--key', nargs=1, type=argparse.FileType('rb'),
                        required=True)
    parser.add_argument('-t', '--type', nargs=1,
                        choices=['rsa', 'xor', 'xor_cbc'],
                        required=True)

    args = parser.parse_args()

    cases = {
        'xor': {
            'enc': xor.encrypt,
            'dec': xor.decrypt,
        },
        'xor_cbc': {
            'enc': xor_cbc.encrypt,
            'dec': xor_cbc.decrypt,
        },
        'rsa': {
            'enc': rsa.encrypt,
            'dec': rsa.decrypt,
        },
    }

    encrypted_text = cases[args.type[0]][args.action[0]](
        args.src.read(), args.key[0].read()
    )
    args.dst.write(encrypted_text)


if __name__ == "__main__":
    _main()
