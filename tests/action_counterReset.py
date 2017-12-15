import time
from pyruse.actions.action_counterReset import Action
from pyruse.actions import action_counterRaise

def whenResetThenCountIs0():
    entry = {"k": "reset#1"}
    resetAct = Action({"counter": "test", "for": "k", "save": "action_counterReset1"})
    raiseAct = action_counterRaise.Action({"counter": "test", "for": "k", "save": "action_counterReset1"})
    raiseAct.act(entry)
    assert entry["action_counterReset1"] == 1
    resetAct.act(entry)
    assert entry["action_counterReset1"] == 0

def whenNoGraceTimeThenRaiseWorks():
    entry = {"k": "reset#2"}
    resetAct = Action({"counter": "test", "for": "k", "save": "action_counterReset2"})
    raiseAct = action_counterRaise.Action({"counter": "test", "for": "k", "save": "action_counterReset2"})
    raiseAct.act(entry)
    assert entry["action_counterReset2"] == 1
    resetAct.act(entry)
    assert entry["action_counterReset2"] == 0
    raiseAct.act(entry)
    assert entry["action_counterReset2"] == 1

def whenGraceTimeThenRaiseFails():
    entry = {"k": "reset#3"}
    resetAct = Action({"counter": "test", "for": "k", "save": "action_counterReset3", "graceSeconds": 1})
    raiseAct = action_counterRaise.Action({"counter": "test", "for": "k", "save": "action_counterReset3"})
    raiseAct.act(entry)
    assert entry["action_counterReset3"] == 1
    resetAct.act(entry)
    assert entry["action_counterReset3"] == 0
    raiseAct.act(entry)
    assert entry["action_counterReset3"] == 0

def whenGraceTimeThenRaiseWorksAtGraceEnd():
    entry = {"k": "reset#4"}
    resetAct = Action({"counter": "test", "for": "k", "save": "action_counterReset4", "graceSeconds": 1})
    raiseAct = action_counterRaise.Action({"counter": "test", "for": "k", "save": "action_counterReset4"})
    raiseAct.act(entry)
    assert entry["action_counterReset4"] == 1
    resetAct.act(entry)
    assert entry["action_counterReset4"] == 0
    raiseAct.act(entry)
    assert entry["action_counterReset4"] == 0
    time.sleep(1)
    raiseAct.act(entry)
    assert entry["action_counterReset4"] == 1
