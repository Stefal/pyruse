# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_pcreAny import Filter

def whenMatchesThenTrue():
    assert Filter({"field": "v", "re": ["cool", "ok"]}).filter({"v": "joke"})

def whenNoMatchThenFalse():
    assert not Filter({"field": "v", "re": ["bad", "ko"]}).filter({"v": "Koala"})

def whenNamedGroupsThenFoundInEntry():
    entry = {"v": "It works or not"}
    Filter({"field": "v", "re": ["^(?P<o>It)(?P<k> works)", "(?P<k>or)(?P<o> not)$"]}).filter(entry)
    assert entry["o"] + entry["k"] == "It works"
