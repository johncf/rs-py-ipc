#!env python3

#import logging
import sys
import json
import socket
import logging

def main():
    line = sys.stdin.readline()
    logging.root.info("read: %s", line)

    port, token = json.loads(line)
    sock = socket.socket()
    sock.connect(("127.0.0.1", port))

    send_json = json.dumps([0, token])
    logging.root.info("send: %s", send_json)
    sock.send("{}\n".format(send_json).encode('utf-8'))

if __name__ == '__main__':
    handler = logging.FileHandler('plug.log', 'w')
    handler.formatter = logging.Formatter(
        '%(asctime)s [%(levelname)s @ '
        '%(filename)s:%(funcName)s:%(lineno)s] - %(message)s')
    logging.root.addHandler(handler)
    logging.root.setLevel(logging.DEBUG)
    main()
