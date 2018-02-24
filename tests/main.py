# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import os
import subprocess
import sys
from datetime import datetime

sys.path.insert(1, "..")
from pyruse import actions, config, module, workflow

def _clean():
    for f in ['acted_on.log', 'action_nftBan.py.json', 'email.dump', 'nftBan.cmd', 'unfiltered.log']:
        if os.path.exists(f):
            os.remove(f)

def main():
    global _microsec
    conf = config.Config(os.curdir)

    # Unit tests
    import filter_equals, filter_greaterOrEquals, filter_in, filter_inNetworks, filter_lowerOrEquals, filter_pcre, filter_pcreAny, filter_userExists
    import action_counterRaise, action_counterReset, action_dailyReport, action_email, action_nftBan

    filter_equals.unitTests()
    filter_greaterOrEquals.unitTests()
    filter_in.unitTests()
    filter_inNetworks.unitTests()
    filter_lowerOrEquals.unitTests()
    filter_pcre.unitTests()
    filter_pcreAny.unitTests()
    filter_userExists.unitTests()
    action_counterRaise.unitTests()
    action_counterReset.unitTests()
    action_dailyReport.unitTests()
    action_email.unitTests()
    action_nftBan.unitTests()

    # Integration test
    wf = workflow.Workflow(conf.asMap().get("actions", {}))
    _microsec = 0
    test = [
        entry("dmz", "ftp", "an ftp message", 0),
        entry("dmz", "login", "Failed password for Unknown User from 1.2.3.4"),
        entry("dmz", "login", "Failed password for nobody from 5.6.7.8"),
        entry("dmz", "login", "End of session for root on localhost"),
        entry("dmz", "login", "Failed password for User Unknown from 1.2.3.4"),
        entry("bck", "ftp", "file requested"),
        entry("dmz", "login", "Accepted password for root from 1.2.3.4"),
        entry("bck", "login", "Failed password for root from 1.2.3.4"),
        entry("bck", "login", "Failed password for nobody from 1.2.3.4"),
        entry("dmz", "login", "Failed password for foobar from 1.2.3.4"),
        entry("dmz", "login", "Failed password for nobody from 5.6.7.8")
    ]
    _clean()
    for e in test:
        run(wf, e)
    actions.action_dailyReport.Action._hour = 25
    run(wf, entry("bck", "login", "Failed password for root from ::1", 11))
    for f in ['acted_on.log', 'email.dump', 'nftBan.cmd', 'unfiltered.log']:
        assert os.path.exists(f), "file should exist: " + f
        try:
            subprocess.run(
                [   "/usr/bin/bash",
                    "-c",
                    "diff -U0 \"$0\"{,.test_ref} | grep -vE '^[-+@^]{2,3} |={5,}[0-9]+=='",
                    f],
                check = True)
            assert False, "differences found in " + f
        except subprocess.CalledProcessError:
            pass # OK, no difference found
    _clean()
    os.remove('action_dailyReport.py.journal')

def entry(host, service, message, microsecond = None):
    global _microsec
    if microsecond:
        _microsec = microsecond
    _microsec += 1
    return {
        "__REALTIME_TIMESTAMP": datetime(2118,1,1,8,1,1,_microsec),
        "_HOSTNAME": host,
        "service": service,
        "MESSAGE": message
    }

def run(workflow, logEntry):
    step = workflow.firstStep
    while step is not None:
        step = step.run(logEntry)

if __name__ == '__main__':
    main()
