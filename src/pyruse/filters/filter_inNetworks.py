# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import socket
from functools import reduce
from pyruse import base

class Filter(base.Filter):
    ipReducer = lambda bits, byte: bits<<8 | byte

    def __init__(self, args):
        super().__init__()
        self.field = args["field"]
        ip4Nets = []
        ip6Nets = []
        for net in args["nets"]:
            if ":" in net:
                ip6Nets.append(self._toNetAndMask(socket.AF_INET6, 128, net))
            else:
                ip4Nets.append(self._toNetAndMask(socket.AF_INET, 32, net))
        self.ip4Nets = ip4Nets
        self.ip6Nets = ip6Nets

    def filter(self, entry):
        if self.field not in entry:
            return False
        ip = entry[self.field]
        if ":" in ip:
            return self._filter(socket.AF_INET6, ip, self.ip6Nets)
        else:
            return self._filter(socket.AF_INET, ip, self.ip4Nets)

    def _filter(self, family, ip, nets):
        for (net, mask) in nets:
            numericIP = self._numericIP(family, ip)
            if numericIP & mask == net:
                return True
        return False

    def _toNetAndMask(self, family, bits, net):
        if "/" in net:
            ip, mask = net.split("/")
        else:
            ip, mask = net, bits
        numericMask = ((1<<int(mask))-1)<<(bits-int(mask))
        numericIP = self._numericIP(family, ip)
        return numericIP & numericMask, numericMask

    def _numericIP(self, family, ipString):
        return reduce(Filter.ipReducer, socket.inet_pton(family, ipString))
