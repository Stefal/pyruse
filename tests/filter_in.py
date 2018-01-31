# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_in import Filter

def whenNotInListThenFalse():
    assert not Filter({"field": "v", "values": [0, "test"]}).filter({"v": 3})

def whenInListSameTypeThenTrue():
    assert Filter({"field": "v", "values": [2]}).filter({"v": 2})

def whenInListDiffTypeThenTrue():
    assert Filter({"field": "v", "values": [2.0]}).filter({"v": 2})

def whenNoFieldThenFalse():
    assert not Filter({"field": "v", "values": [0]}).filter({"other": 0})

def unitTests():
    whenNotInListThenFalse()
    whenInListSameTypeThenTrue()
    whenInListDiffTypeThenTrue()
    whenNoFieldThenFalse()
