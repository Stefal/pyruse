# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_equals import Filter

def whenGreaterThenFalse():
    assert not Filter({"field": "v", "value": 2}).filter({"v": 3})

def whenEqualSameTypeThenTrue():
    assert Filter({"field": "v", "value": 2}).filter({"v": 2})

def whenEqualDiffTypeThenTrue():
    assert Filter({"field": "v", "value": 2.0}).filter({"v": 2})

def whenLowerThenFalse():
    assert not Filter({"field": "v", "value": 2}).filter({"v": 0})

def unitTests():
    whenGreaterThenFalse()
    whenEqualSameTypeThenTrue()
    whenEqualDiffTypeThenTrue()
    whenLowerThenFalse()
