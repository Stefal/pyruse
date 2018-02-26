# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from unittest.mock import patch
from pyruse import log
from pyruse.actions.action_log import Action

@patch('pyruse.actions.action_log.log.log')
def whenLogThenRightSystemdCall(mockLog):
    for level in log.Level:
        Action({"level": level.name, "message": "Test: {text}"}).act({"text": "test message"})
        mockLog.assert_called_with(level, "Test: test message")

def unitTests():
    whenLogThenRightSystemdCall()
