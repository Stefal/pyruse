# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from datetime import datetime
from pyruse import dnat
from pyruse.actions.action_dnatCapture import Action

def whenNoSaddrThenError():
    try:
        Action(dict(addr=1))
    except Exception:
        return
    assert False, "An exception should be raised when saddr is absent"

def whenNoAddrNorAddrvalueThenError():
    try:
        Action(dict(saddr=1))
    except Exception:
        return
    assert False, "An exception should be raised when addr and addrValue are absent"

def whenNoAddrButAddrvalueThenNoError():
    Action(dict(saddr=1, addrValue=1))

def whenNoAddrvalueButAddrThenNoError():
    Action(dict(saddr=1, addr=1))

def whenNoKeepsecondsThen6bits():
    a = Action(dict(saddr=1, addr=1))
    assert a.keepBits == 6, "Default keepSeconds (63) should be on 6 bits, not " + str(a.keepBits)

def whenKeepsecondsIs150Then8bits():
    a = Action(dict(saddr=1, addr=1, keepSeconds=150))
    assert a.keepBits == 8, "150 for keepSeconds should be on 8 bits, not " + str(a.keepBits)

def whenInsufficientEntryThenNoMapping():
    dnat._mappings = []
    Action({"saddr": "sa", "addrValue": "x"}).act({"__REALTIME_TIMESTAMP": datetime(2018,1,1)})
    assert dnat._mappings == [], "Got:\n" + str(dnat._mappings) + "\ninstead of []"

def whenFieldAndOrValueThenCheckMapping(spec, entryWithAddr, entryWithDAddr, expect):
    dnat._mappings = []

    # specify the Action
    spec.update({"saddr": "sa"})

    # prepare the entry
    entry = {
        "__REALTIME_TIMESTAMP": datetime(2018,1,1),
        "sa": "vsa", "sp": "vsp"}
    if entryWithAddr:
        entry.update({"a":  "va",  "p":  "vp"})
    if entryWithDAddr:
        entry.update({"da": "vda", "dp": "vdp"})

    # run
    Action(spec).act(entry)

    # check the result
    expect.update({"bits": 6, "time": 23668144, "saddr": "vsa"})
    assert dnat._mappings == [expect], "Got:\n" + str(dnat._mappings) + "\ninstead of:\n" + str([expect])

def unitTests():
    whenNoSaddrThenError()
    whenNoAddrNorAddrvalueThenError()
    whenNoAddrButAddrvalueThenNoError()
    whenNoAddrvalueButAddrThenNoError()
    whenNoKeepsecondsThen6bits()
    whenKeepsecondsIs150Then8bits()
    whenInsufficientEntryThenNoMapping()

    whenFieldAndOrValueThenCheckMapping({"addr": "a"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": None, "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addrValue": "x"}, True, True,
        {"sport": None, "addr": "x", "port": None, "daddr": None, "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "addrValue": "x"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": None, "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "addrValue": "x"}, False, True,
        {"sport": None, "addr": "x", "port": None, "daddr": None, "dport": None})

    whenFieldAndOrValueThenCheckMapping({"addr": "a", "daddr": "da"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": "vda", "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "daddrValue": "x"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": "x", "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "daddr": "da", "daddrValue": "x"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": "vda", "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "daddr": "da", "daddrValue": "x"}, True, False,
        {"sport": None, "addr": "va", "port": None, "daddr": "x", "dport": None})

    whenFieldAndOrValueThenCheckMapping({"addr": "a", "port": "p"}, True, True,
        {"sport": None, "addr": "va", "port": "vp", "daddr": None, "dport": None})
    whenFieldAndOrValueThenCheckMapping({"addr": "a", "dport": "dp"}, True, True,
        {"sport": None, "addr": "va", "port": None, "daddr": None, "dport": "vdp"})
