import os
import re
from pyruse.actions.action_email import Action
from pyruse import config

mail_filename = "email.dump"

def whenEmailWithSubjectThenCheckContents():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action({"subject": "Test1", "message": "TestMsg{m}"}).act({"m": "#1"})
    assert os.path.exists(mail_filename)
    conf = config.Config().asMap().get("email", {})
    reSubject = re.compile(r"^Subject: (.*)")
    reFrom = re.compile(r"^From: (.*)")
    reTo = re.compile(r"^To: (.*)")
    subjOK = False
    fromOK = False
    toOK = False
    nbMsg = 0
    with open(mail_filename, 'rt') as m:
        for line in m:
            match = reSubject.match(line)
            if match:
                subjOK = match.group(1) == "Test1"
            match = reFrom.match(line)
            if match:
                fromOK = match.group(1) == conf.get("from", "pyruse")
            match = reTo.match(line)
            if match:
                toOK = match.group(1).split(", ") == conf.get("to", ["hostmaster"])
            if "TestMsg#1" in line:
                nbMsg += 1
    assert subjOK
    assert fromOK
    assert toOK
    assert nbMsg == 1
    os.remove(mail_filename)

def whenEmailWithoutSubjectThenCheckContents():
    if os.path.exists(mail_filename):
        os.remove(mail_filename)
    Action({"message": "TestMsg{m}"}).act({"m": "#2"})
    assert os.path.exists(mail_filename)
    conf = config.Config().asMap().get("email", {})
    reSubject = re.compile(r"^Subject: (.*)")
    reFrom = re.compile(r"^From: (.*)")
    reTo = re.compile(r"^To: (.*)")
    subjOK = False
    fromOK = False
    toOK = False
    nbMsg = 0
    with open(mail_filename, 'rt') as m:
        for line in m:
            match = reSubject.match(line)
            if match:
                subjOK = match.group(1) == "Pyruse Notification"
            match = reFrom.match(line)
            if match:
                fromOK = match.group(1) == conf.get("from", "pyruse")
            match = reTo.match(line)
            if match:
                toOK = match.group(1).split(", ") == conf.get("to", ["hostmaster"])
            if "TestMsg#2" in line:
                nbMsg += 1
    assert subjOK
    assert fromOK
    assert toOK
    assert nbMsg == 1
    os.remove(mail_filename)
