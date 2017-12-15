import json
import os
import time
from pyruse.actions.action_nftBan import Action

nftBanCmd = "nftBan.cmd"
nftBanState = "action_nftBan.py.json"

def _clean():
    if os.path.exists(nftBanCmd):
        os.remove(nftBanCmd)
    if os.path.exists(nftBanState):
        os.remove(nftBanState)

def whenBanIPv4ThenAddToIPv4Set():
    _clean()
    Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban"}).act({"ip": "10.0.0.1"})
    assert os.path.exists(nftBanCmd)
    assert os.path.exists(nftBanState)
    nbLines = 0
    with open(nftBanCmd, "rt") as c:
        for line in c:
            assert line == "add element I4 ban {10.0.0.1}\n", line
            nbLines += 1
    assert nbLines == 1, nbLines
    nbBans = 0
    with open(nftBanState) as s:
        for ban in json.load(s):
            assert ban["IP"] == "10.0.0.1" and ban["nftSet"] == "I4 ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenBanIPv6ThenAddToIPv6Set():
    _clean()
    Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban"}).act({"ip": "::1"})
    assert os.path.exists(nftBanCmd)
    assert os.path.exists(nftBanState)
    nbLines = 0
    with open(nftBanCmd, "rt") as c:
        for line in c:
            assert line == "add element I6 ban {::1}\n", line
            nbLines += 1
    assert nbLines == 1, nbLines
    nbBans = 0
    with open(nftBanState) as s:
        for ban in json.load(s):
            assert ban["IP"] == "::1" and ban["nftSet"] == "I6 ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenBanTwoIPThenTwoLinesInState():
    _clean()
    action = Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban"})
    action.act({"ip": "10.0.0.1"})
    action.act({"ip": "::1"})
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(nftBanState)
    nbBans = 0
    with open(nftBanState) as s:
        for ban in json.load(s):
            if ban["IP"] == "10.0.0.1":
                assert ban["nftSet"] == "I4 ban", str(ban)
            elif ban["IP"] == "::1":
                assert ban["nftSet"] == "I6 ban", str(ban)
            else:
                assert false, str(ban)
            nbBans += 1
    assert nbBans == 2, nbBans
    _clean()

def whenBanAnewThenNoDuplicate():
    _clean()
    action = Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban"})
    action.act({"ip": "10.0.0.1"})
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(nftBanCmd)
    assert os.path.exists(nftBanState)
    lineCount = 0
    with open(nftBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add element I4 ban {10.0.0.1}\n", line
            elif lineCount == 2:
                assert line == "delete element I4 ban {10.0.0.1}\n", line
            elif lineCount == 3:
                assert line == "add element I4 ban {10.0.0.1}\n", line
    assert lineCount == 3, lineCount
    nbBans = 0
    with open(nftBanState) as s:
        for ban in json.load(s):
            if ban["IP"] == "10.0.0.1":
                assert ban["nftSet"] == "I4 ban", str(ban)
            nbBans += 1
    assert nbBans == 1, nbBans
    _clean()

def whenFinishedBanThenAsIfNotThere():
    _clean()
    action = Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban", "banSeconds": 1})
    action.act({"ip": "10.0.0.1"})
    time.sleep(1)
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(nftBanCmd)
    lineCount = 0
    with open(nftBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add element I4 ban {10.0.0.1 timeout 1s}\n", line
            elif lineCount == 2:
                assert line == "add element I4 ban {10.0.0.1 timeout 1s}\n", line
    assert lineCount == 2, lineCount
    _clean()

def whenUnfinishedBanThenTimeoutReset():
    _clean()
    action = Action({"IP": "ip", "nftSetIPv4": "I4 ban", "nftSetIPv6": "I6 ban", "banSeconds": 2})
    action.act({"ip": "10.0.0.1"})
    time.sleep(1)
    action.act({"ip": "10.0.0.1"})
    assert os.path.exists(nftBanCmd)
    lineCount = 0
    with open(nftBanCmd, "rt") as c:
        for line in c:
            lineCount += 1
            if lineCount == 1:
                assert line == "add element I4 ban {10.0.0.1 timeout 2s}\n", line
            elif lineCount == 2:
                assert line == "delete element I4 ban {10.0.0.1}\n", line
            elif lineCount == 3:
                assert line == "add element I4 ban {10.0.0.1 timeout 2s}\n", line
    assert lineCount == 3, lineCount
    _clean()
