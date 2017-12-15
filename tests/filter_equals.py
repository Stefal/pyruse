from pyruse.filters.filter_equals import Filter

def whenGreaterThenFalse():
    assert not Filter({"field": "v", "value": 2}).filter({"v": 3})

def whenEqualSameTypeThenTrue():
    assert Filter({"field": "v", "value": 2}).filter({"v": 2})

def whenEqualDiffTypeThenTrue():
    assert Filter({"field": "v", "value": 2.0}).filter({"v": 2})

def whenLowerThenFalse():
    assert not Filter({"field": "v", "value": 2}).filter({"v": 0})
