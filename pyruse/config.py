# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import json
import os
from collections import OrderedDict
from pyruse import log

class Config:
    CONF_NAME = "pyruse.conf"
    _paths = None

    # __main__ must be the first to create a Config object, then paths is remembered
    def __init__(self, paths = None):
        if paths is None:
            paths = Config._paths
        Config._paths = paths
        conf = None
        for p in paths:
            try:
                with open(os.path.join(p, "pyruse.json")) as conffile:
                    conf = json.load(conffile, object_pairs_hook = OrderedDict)
            except IOError:
                log.debug("IOError while opening %s\n" % conffile)
            except json.JSONDecodeError:
                log.debug("JSONDecodeError while opening %s\n" % conffile)
        if conf is None:
            raise FileNotFoundError("File `%s` not found in either of %s." \
                % (Config.CONF_NAME, paths))
        self.conf = conf

    def asMap(self):
        return self.conf
