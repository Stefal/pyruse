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

def newEntry(m):
    return {"__REALTIME_TIMESTAMP": datetime.utcnow(), "m": m}

def whenNewDayThenReport():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action._hour = 0
    oAction.act(newEntry("message1"))
    assert not os.path.exists(mail_filename)
    Action._hour = 25
    oAction.act(newEntry("message2"))
    assert os.path.exists(mail_filename)
    os.remove(mail_filename)

def whenEmailThenCheckContents():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action._hour = 0
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

def whenReportThenNewSetOfMessages():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action._hour = 25
    oAction.act(newEntry("message3"))
    assert os.path.exists(mail_filename)
    os.remove(mail_filename)
    whenEmailThenCheckContents()

def unitTests():
    whenNewDayThenReport()
    whenEmailThenCheckContents()
    whenReportThenNewSetOfMessages()
