# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import os
import re
from datetime import datetime
from pyruse.actions.action_dailyReport import Action
from pyruse import config

mail_filename = "email.dump"
wAction = Action({"level": "WARN", "message": "WarnMsg {m}"})
iAction = Action({"level": "INFO", "message": "InfoMsg {m}"})
oAction = Action({"level": "OTHER", "message": "MiscMsg {m}"})
wActFirst = Action({"level": "WARN", "message": "WarnMsg {m}", "details": "FIRST"})
wActLast = Action({"level": "WARN", "message": "WarnMsg {m}", "details": "LAST"})
wActFL = Action({"level": "WARN", "message": "WarnMsg {m}", "details": "FIRSTLAST"})
wActNone = Action({"level": "WARN", "message": "WarnMsg {m}", "details": "NONE"})

def newEntry(m):
    return {"__REALTIME_TIMESTAMP": datetime.utcnow(), "m": m}

def whenNewDayThenReport():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    oAction.act(newEntry("message1"))
    assert not os.path.exists(mail_filename)
    Action._hour = 25
    oAction.act(newEntry("message2"))
    assert os.path.exists(mail_filename)
    os.remove(mail_filename)

def whenEmailThenCheck3Sections():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    wAction.act(newEntry("messageW"))
    iAction.act(newEntry("messageI"))
    Action._hour = 25
    oAction.act(newEntry("messageO"))
    assert os.path.exists(mail_filename)
    conf = config.Config().asMap().get("email", {})
    reSubject = re.compile(r"^Subject: (.*)")
    reFrom = re.compile(r"^From: (.*)")
    reTo = re.compile(r"^To: (.*)")
    subjOK = False
    fromOK = False
    toOK = False
    nbWarn = 0
    nbInfo = 0
    nbMisc = 0
    with open(mail_filename, 'rt') as m:
        for line in m:
            match = reSubject.match(line)
            if match:
                subjOK = match.group(1) == conf.get("subject", "Pyruse Report")
            match = reFrom.match(line)
            if match:
                fromOK = match.group(1) == conf.get("from", "pyruse")
            match = reTo.match(line)
            if match:
                toOK = match.group(1).split(", ") == conf.get("to", ["hostmaster"])
            if "WarnMsg" in line:
                nbWarn += 1
            if "InfoMsg" in line:
                nbInfo += 1
            if "MiscMsg" in line:
                nbMisc += 1
    assert subjOK
    assert fromOK
    assert toOK
    assert nbWarn == 2
    assert nbInfo == 2
    assert nbMisc == 2
    os.remove(mail_filename)

def _compareEmailWithExpected(expected):
    assert os.path.exists(mail_filename)
    reTime = re.compile(r"\d{4}(?:[- :.]\d{2}){6}\d{4}")
    warnSeen = False
    nbTimes = 0
    nbFirst = 0
    nbLast = 0
    line = ""
    with open(mail_filename, 'rt') as m:
        for l in m:
            if l != "" and l[-1:] == "=":
                line += l[:-1]
                continue
            elif l == "" and warnSeen:
                break
            line += l
            if "WarnMsg" in line:
                warnSeen = True
            elif not warnSeen:
                line = ""
                continue
            nbTimes += len(reTime.findall(line))
            if "From=C2=A0:" in line:
                nbFirst += 1
            if "Until:" in line:
                nbLast += 1
            if "</tr>" in line:
                break
            line = ""
    seen = dict(warn = warnSeen, times = nbTimes, first = nbFirst, last = nbLast)
    assert seen == expected, "Expected=" + str(expected) + " ≠ Seen=" + str(seen)
    os.remove(mail_filename)

def whenEmailThenCheckTimes(warnAction, expected):
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    warnAction.act(newEntry("messageW"))
    warnAction.act(newEntry("messageW"))
    Action._hour = 25
    warnAction.act(newEntry("messageW"))
    _compareEmailWithExpected(expected)

def whenSeveralDetailsModesThenOnlyOneWarn():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    wAction.act(newEntry("messageW"))
    wAction.act(newEntry("messageW"))
    wAction.act(newEntry("messageW"))
    wAction.act(newEntry("messageW"))
    wAction.act(newEntry("messageW"))
    wActFirst.act(newEntry("messageW"))
    wActFirst.act(newEntry("messageW"))
    wActFirst.act(newEntry("messageW"))
    wActLast.act(newEntry("messageW"))
    wActLast.act(newEntry("messageW"))
    wActLast.act(newEntry("messageW"))
    wActFL.act(newEntry("messageW"))
    wActFL.act(newEntry("messageW"))
    wActFL.act(newEntry("messageW"))
    wActFL.act(newEntry("messageW"))
    wActNone.act(newEntry("messageW"))
    wActNone.act(newEntry("messageW"))
    wActNone.act(newEntry("messageW"))
    Action._hour = 25
    wActNone.act(newEntry("messageW"))
    _compareEmailWithExpected(dict(warn = True, times = 9, first = 2, last = 2))

def whenReportThenNewSetOfMessages():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action._hour = 25
    oAction.act(newEntry("message3"))
    assert os.path.exists(mail_filename)
    os.remove(mail_filename)
    whenEmailThenCheck3Sections()

def unitTests():
    whenNewDayThenReport()
    whenEmailThenCheck3Sections()
    whenEmailThenCheckTimes(wActFirst, dict(warn = True, times = 1, first = 1, last = 0))
    whenEmailThenCheckTimes(wActLast, dict(warn = True, times = 1, first = 0, last = 1))
    whenEmailThenCheckTimes(wActFL, dict(warn = True, times = 2, first = 1, last = 1))
    whenEmailThenCheckTimes(wActNone, dict(warn = True, times = 0, first = 0, last = 0))
    whenSeveralDetailsModesThenOnlyOneWarn()
    whenReportThenNewSetOfMessages()
