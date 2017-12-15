from pyruse.filters.filter_userExists import Filter

def whenUserExistsThenTrue():
    assert Filter({"field": "user"}).filter({"user": "root"})

def whenGarbageThenFalse():
    assert not Filter({"field": "user"}).filter({"user": "auietsnr"})
