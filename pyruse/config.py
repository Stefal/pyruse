# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import json
import os
from collections import OrderedDict
from pyruse import log

class Config:
    CONF_NAME = "pyruse.json"
    _paths = None

    # __main__ must be the first to create a Config object, then paths are remembered
    def __init__(self, paths = None):
        if paths is None:
            paths = Config._paths
        Config._paths = paths
        for p in paths:
            confpath = os.path.join(p, Config.CONF_NAME)
            try:
                with open(confpath) as conffile:
                    conf = json.load(conffile, object_pairs_hook = OrderedDict)
                    self.conf = conf
                    break
            except IOError:
                log.debug("IOError while opening %s\n" % confpath)
            except json.JSONDecodeError:
                log.debug("JSONDecodeError while opening %s\n" % confpath)
        else:
            raise FileNotFoundError("File `%s` not found in either of %s." \
                % (Config.CONF_NAME, str(paths)))

    def asMap(self):
        return self.conf
