# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_pcre import Filter

def whenMatchesThenTrue():
    assert Filter({"field": "v", "re": "ok"}).filter({"v": "joke"})

def whenNoMatchThenFalse():
    assert not Filter({"field": "v", "re": "ko"}).filter({"v": "Koala"})

def whenSaveThenGroupsInEntry():
    entry = {"v": "yet another test"}
    Filter({"field": "v", "re": "^(.).* .*(.)r .*(.).$", "save": [ "y", "e", "s" ]}).filter(entry)
    assert entry["y"] + entry["e"] + entry["s"] == "yes"

def whenNamedGroupsThenFoundInEntry():
    entry = {"v": "yet another test"}
    Filter({"field": "v", "re": "^(?P<y>.).* .*(?P<e>.)r .*(?P<s>.).$"}).filter(entry)
    assert entry["y"] + entry["e"] + entry["s"] == "yes"

def unitTests():
    whenMatchesThenTrue()
    whenNoMatchThenFalse()
    whenSaveThenGroupsInEntry()
    whenNamedGroupsThenFoundInEntry()
