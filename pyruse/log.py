# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from systemd import journal

EMERG = 0 # System is unusable.
ALERT = 1 # Action must be taken immediately.
CRIT = 2 # Critical conditions, such as hard device errors.
ERR = 3 # Error conditions.
WARNING = 4 # Warning conditions.
NOTICE = 5 # Normal but significant conditions.
INFO = 6 # Informational messages.
DEBUG = 7

def log(level, string):
	journal.send(string, PRIORITY = level)

def debug(string):
	global DEBUG
	log(DEBUG, string)

def error(string):
	global ERR
	log(ERR, string)
