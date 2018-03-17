# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import json
import os
import time
from pyruse.actions.action_ipsetBan import Action

ipBanCmd = "ipsetBan.cmd"
ipBanState = "action_ipsetBan.py.json"

def _clean():
    if os.path.exists(ipBanCmd):
        os.remove(ipBanCmd)
    if os.path.exists(ipBanState):
        os.remove(ipBanState)

def whenBanIPv4ThenAddToIPv4Set():
    _clean()
    Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban"}).act({"ip": "10.0.0.1"})
    assert os.path.exists(ipBanCmd)
    assert os.path.exists(ipBanState)
    nbLines = 0
    with open(ipBanCmd, "rt") as c:
        for line in c:
            assert line == "add I4ban 10.0.0.1\n", line
            nbLines += 1
    assert nbLines == 1, nbLines
    nbBans = 0
    with open(ipBanState) as s:
        for ban in json.load(s):
            assert ban["IP"] == "10.0.0.1" and ban["nfSet"] == "I4ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenBanIPv6ThenAddToIPv6Set():
    _clean()
    Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban"}).act({"ip": "::1"})
    assert os.path.exists(ipBanCmd)
    assert os.path.exists(ipBanState)
    nbLines = 0
    with open(ipBanCmd, "rt") as c:
        for line in c:
            assert line == "add I6ban ::1\n", line
            nbLines += 1
    assert nbLines == 1, nbLines
    nbBans = 0
    with open(ipBanState) as s:
        for ban in json.load(s):
            assert ban["IP"] == "::1" and ban["nfSet"] == "I6ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenBanTwoIPThenTwoLinesInState():
    _clean()
    action = Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban"})
    action.act({"ip": "10.0.0.1"})
    action.act({"ip": "::1"})
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(ipBanState)
    nbBans = 0
    with open(ipBanState) as s:
        for ban in json.load(s):
            if ban["IP"] == "10.0.0.1":
                assert ban["nfSet"] == "I4ban", str(ban)
            elif ban["IP"] == "::1":
                assert ban["nfSet"] == "I6ban", str(ban)
            else:
                assert false, str(ban)
            nbBans += 1
    assert nbBans == 2, nbBans
    _clean()

def whenBanAnewThenNoDuplicate():
    _clean()
    action = Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban"})
    action.act({"ip": "10.0.0.1"})
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(ipBanCmd)
    assert os.path.exists(ipBanState)
    lineCount = 0
    with open(ipBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add I4ban 10.0.0.1\n", line
            elif lineCount == 2:
                assert line == "del I4ban 10.0.0.1\n", line
            elif lineCount == 3:
                assert line == "add I4ban 10.0.0.1\n", line
    assert lineCount == 3, lineCount
    nbBans = 0
    with open(ipBanState) as s:
        for ban in json.load(s):
            if ban["IP"] == "10.0.0.1":
                assert ban["nfSet"] == "I4ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenFinishedBanThenAsIfNotThere():
    _clean()
    action = Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban", "banSeconds": 1})
    action.act({"ip": "10.0.0.1"})
    time.sleep(1)
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(ipBanCmd)
    lineCount = 0
    with open(ipBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add I4ban 10.0.0.1 timeout 1\n", line
            elif lineCount == 2:
                assert line == "add I4ban 10.0.0.1 timeout 1\n", line
    assert lineCount == 2, lineCount
    _clean()

def whenUnfinishedBanThenTimeoutReset():
    _clean()
    action = Action({"IP": "ip", "ipSetIPv4": "I4ban", "ipSetIPv6": "I6ban", "banSeconds": 2})
    action.act({"ip": "10.0.0.1"})
    time.sleep(1)
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(ipBanCmd)
    lineCount = 0
    with open(ipBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add I4ban 10.0.0.1 timeout 2\n", line
            elif lineCount == 2:
                assert line == "del I4ban 10.0.0.1\n", line
            elif lineCount == 3:
                assert line == "add I4ban 10.0.0.1 timeout 2\n", line
    assert lineCount == 3, lineCount
    _clean()

def unitTests():
    whenBanIPv4ThenAddToIPv4Set()
    whenBanIPv6ThenAddToIPv6Set()
    whenBanTwoIPThenTwoLinesInState()
    whenBanAnewThenNoDuplicate()
    whenFinishedBanThenAsIfNotThere()
    whenUnfinishedBanThenTimeoutReset()
