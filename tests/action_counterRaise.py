# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import time
from pyruse.actions.action_counterRaise import Action
from pyruse.actions import action_counterReset

def whenNonExistingThenRaiseTo1():
    entry = {"k": "raise#1"}
    Action({"counter": "test", "for": "k", "save": "action_counterRaise1"}).act(entry)
    assert entry["action_counterRaise1"] == 1

def whenKeepSecondsThenRaiseUntilTimeOut():
    entry = {"k": "raise#2"}
    action = Action({"counter": "test", "for": "k", "save": "action_counterRaise2", "keepSeconds": 3})
    action.act(entry)
    assert entry["action_counterRaise2"] == 1
    time.sleep(2)
    action.act(entry)
    assert entry["action_counterRaise2"] == 2
    time.sleep(2)
    action.act(entry)
    assert entry["action_counterRaise2"] == 2 # one tick timed out

def whenDifferentKeyThenDifferentCounter():
    entry1 = {"k": "raise#3"}
    entry2 = {"k": "raise#4"}
    action = Action({"counter": "test", "for": "k", "save": "action_counterRaise3"})
    action.act(entry1)
    assert entry1["action_counterRaise3"] == 1
    action.act(entry2)
    assert entry2["action_counterRaise3"] == 1
    action.act(entry2)
    assert entry2["action_counterRaise3"] == 2
    action.act(entry2)
    assert entry2["action_counterRaise3"] == 3
    action.act(entry1)
    assert entry1["action_counterRaise3"] == 2

def whenGraceTimeThenCountIs0():
    entry = {"k": "raise#5"}
    raiseAct = Action({"counter": "test", "for": "k", "save": "action_counterRaise4"})
    graceAct = action_counterReset.Action({"counter": "test", "for": "k", "graceSeconds": 1, "save": "action_counterRaise4"})
    raiseAct.act(entry)
    assert entry["action_counterRaise4"] == 1
    graceAct.act(entry)
    assert entry["action_counterRaise4"] == 0
    raiseAct.act(entry)
    assert entry["action_counterRaise4"] == 0
    time.sleep(1)
    raiseAct.act(entry)
    assert entry["action_counterRaise4"] == 1

def unitTests():
    whenNonExistingThenRaiseTo1()
    whenKeepSecondsThenRaiseUntilTimeOut()
    whenDifferentKeyThenDifferentCounter()
    whenGraceTimeThenCountIs0()
