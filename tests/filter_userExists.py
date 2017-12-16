# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_userExists import Filter

def whenUserExistsThenTrue():
    assert Filter({"field": "user"}).filter({"user": "root"})

def whenGarbageThenFalse():
    assert not Filter({"field": "user"}).filter({"user": "auietsnr"})
