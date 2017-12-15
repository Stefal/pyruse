from pyruse.filters.filter_greaterOrEquals import Filter

def whenGreaterPosIntThenTrue():
    assert Filter({"field": "v", "value": 2}).filter({"v": 3})

def whenGreaterNegFloatThenTrue():
    assert Filter({"field": "v", "value": -2.1}).filter({"v": -1.9})

def whenEqualSameTypeThenTrue():
    assert Filter({"field": "v", "value": 2}).filter({"v": 2})

def whenEqualDiffTypeThenTrue():
    assert Filter({"field": "v", "value": 2.0}).filter({"v": 2})

def whenLowerThenFalse():
    assert not Filter({"field": "v", "value": 2}).filter({"v": 0})
