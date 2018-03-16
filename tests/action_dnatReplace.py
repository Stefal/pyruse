# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from datetime import datetime
from pyruse import dnat
from pyruse.actions.action_dnatReplace import Action

def whenNoSaddrintoThenError():
    try:
        Action(dict(addr=1))
    except Exception:
        return
    assert False, "An exception should be raised when saddrInto is absent"

def whenNoMatchFieldThenError():
    try:
        Action(dict(saddrInto=1))
    except Exception:
        return
    assert False, "An exception should be raised when no match-field is present"

def whenSaddrintoAndAtLeastOneMatchFieldThenNoError():
    a = Action(dict(saddrInto=1, dport=1))
    assert a.matchers == [(1, "dport")], "Got:\n" + str(a.matchers) + "\ninstead of:\n" + str([(1, "dport")])
    assert a.updaters == [(1, "saddr")], "Got:\n" + str(a.updaters) + "\ninstead of:\n" + str([(1, "saddr")])

def whenNoMatchingEntryThenNoChange():
    dnat._mappings = [{
        "bits": 7, "time": 1183407200,
        "saddr": "bad",  "sport": None,
        "addr": "prox",  "port": 12345,
        "daddr": "serv", "dport": None}]
    a = Action(dict(saddrInto="sa", port="sp"))

    entryIn = dict(sa = "prox", da = "serv")
    entryOut = entryIn.copy()
    a.act(entryOut)
    assert entryIn == entryOut, "Got:\n" + str(entryOut) + "\ninstead of:\n" + str(entryIn)

def whenNoMatchingValueThenNoChange():
    dnat._mappings = [{
        "bits": 7, "time": 1183407200,
        "saddr": "bad",  "sport": None,
        "addr": "prox",  "port": 12345,
        "daddr": "serv", "dport": None}]
    a = Action(dict(saddrInto="sa", port="sp"))

    entryIn = dict(sa = "prox", sp = 1234, da = "serv")
    entryOut = entryIn.copy()
    a.act(entryOut)
    assert entryIn == entryOut, "Got:\n" + str(entryOut) + "\ninstead of:\n" + str(entryIn)

def whenMatchingEntryThenChange():
    dnat._mappings = [{
        "bits": 7, "time": 1183407200,
        "saddr": "bad",  "sport": None,
        "addr": "prox",  "port": 12345,
        "daddr": "serv", "dport": None}]
    a = Action(dict(saddrInto="sa", port="sp"))

    entryIn = dict(sa = "prox", sp = 12345, da = "serv")
    expect = entryIn.copy()
    expect.update({"sa": "bad"})
    entryOut = entryIn.copy()
    a.act(entryOut)
    assert expect == entryOut, "Got:\n" + str(entryOut) + "\ninstead of:\n" + str(expect)

def unitTests():
    whenNoSaddrintoThenError()
    whenNoMatchFieldThenError()
    whenSaddrintoAndAtLeastOneMatchFieldThenNoError()
    whenNoMatchingEntryThenNoChange()
    whenNoMatchingValueThenNoChange()
    whenMatchingEntryThenChange()
