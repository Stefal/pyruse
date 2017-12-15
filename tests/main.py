import os
import subprocess
import sys
from datetime import datetime

sys.path.insert(1, "..")
from pyruse import actions, base, config, module, workflow

def _clean():
    for f in ['acted_on.log', 'action_nftBan.py.json', 'email.dump', 'nftBan.cmd', 'unfiltered.log']:
        if os.path.exists(f):
            os.remove(f)

def main():
    global _microsec
    conf = config.Config(os.curdir)
    fback = conf.asMap().get("fallback", {})
    if "all_filters_failed" in fback:
        base.filterFallback = module.get(fback.get("all_filters_failed")).module
    else:
        base.filterFallback = None
    if "finalize_after_last_action" in fback:
        base.actionFallback = module.get(fback.get("finalize_after_last_action")).module
    else:
        base.actionFallback = None

    # Unit tests
    import filter_equals, filter_greaterOrEquals, filter_pcre, filter_userExists
    import action_counterRaise, action_counterReset, action_dailyReport, action_email, action_nftBan

    filter_equals.whenGreaterThenFalse()
    filter_equals.whenEqualSameTypeThenTrue()
    filter_equals.whenEqualDiffTypeThenTrue()
    filter_equals.whenLowerThenFalse()

    filter_greaterOrEquals.whenGreaterPosIntThenTrue()
    filter_greaterOrEquals.whenGreaterNegFloatThenTrue()
    filter_greaterOrEquals.whenEqualSameTypeThenTrue()
    filter_greaterOrEquals.whenEqualDiffTypeThenTrue()
    filter_greaterOrEquals.whenLowerThenFalse()

    filter_pcre.whenMatchesThenTrue()
    filter_pcre.whenNoMatchThenFalse()
    filter_pcre.whenSaveThenGroupsInEntry()
    filter_pcre.whenNamedGroupsThenFoundInEntry()

    filter_userExists.whenUserExistsThenTrue()
    filter_userExists.whenGarbageThenFalse()

    action_counterRaise.whenNonExistingThenRaiseTo1()
    action_counterRaise.whenKeepSecondsThenRaiseUntilTimeOut()
    action_counterRaise.whenDifferentKeyThenDifferentCounter()
    action_counterRaise.whenGraceTimeThenCountIs0()

    action_counterReset.whenResetThenCountIs0()
    action_counterReset.whenNoGraceTimeThenRaiseWorks()
    action_counterReset.whenGraceTimeThenRaiseFails()
    action_counterReset.whenGraceTimeThenRaiseWorksAtGraceEnd()

    action_dailyReport.whenNewDayThenReport()
    action_dailyReport.whenEmailThenCheckContents()
    action_dailyReport.whenReportThenNewSetOfMessages()

    action_email.whenEmailWithSubjectThenCheckContents()
    action_email.whenEmailWithoutSubjectThenCheckContents()

    action_nftBan.whenBanIPv4ThenAddToIPv4Set()
    action_nftBan.whenBanIPv6ThenAddToIPv6Set()
    action_nftBan.whenBanTwoIPThenTwoLinesInState()
    action_nftBan.whenBanAnewThenNoDuplicate()
    action_nftBan.whenFinishedBanThenAsIfNotThere()
    action_nftBan.whenUnfinishedBanThenTimeoutReset()

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
        wf.run(e)
    actions.action_dailyReport.Action._hour = 25
    wf.run(entry("bck", "login", "Failed password for root from ::1", 11))
    for f in ['acted_on.log', 'email.dump', 'nftBan.cmd', 'unfiltered.log']:
        assert os.path.exists(f)
        try:
            subprocess.run(
                [   "/usr/bin/bash",
                    "-c",
                    "diff -U0 \"$0\"{,.test_ref} | grep -vE '^[-+@^]{2,3} |={5,}[0-9]+=='",
                    f],
                check = True)
            assert false, "differences found in " + f
        except subprocess.CalledProcessError:
            pass # OK, no difference found
    _clean()

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

if __name__ == '__main__':
    main()
