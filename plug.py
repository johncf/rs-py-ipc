#!env python3

import sys
import json
import socket
import logging

logger = logging.root

def main():
    line = sys.stdin.readline()
    logger.info("read: %s", line)

    port, token = json.loads(line)
    sock = socket.socket()
    sock.connect(("127.0.0.1", port))

    send_json = json.dumps([0, token])
    logger.info("send: %s", send_json)
    sock.send("{}\n".format(send_json).encode('utf-8'))

if __name__ == '__main__':
    handler = logging.FileHandler('plug.log', 'w')
    handler.formatter = logging.Formatter(
        '%(asctime)s [%(levelname)s @ '
        '%(filename)s:%(funcName)s:%(lineno)s] - %(message)s',
        datefmt='%H:%M:%S')
    logger.addHandler(handler)
    logger.setLevel(logging.DEBUG)
    main()
