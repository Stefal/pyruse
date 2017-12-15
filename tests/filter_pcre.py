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
