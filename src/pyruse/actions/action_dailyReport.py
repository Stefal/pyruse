# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import json
import os
import string
from collections import OrderedDict
from datetime import datetime
from enum import Enum, unique
from pyruse import base, config, email

@unique
class Details(Enum):
    NONE      = [ lambda l: [] ]
    FIRST     = [ lambda l: ["From : " + str(t) for t in l[:1]]  ]
    LAST      = [ lambda l: ["Until: " + str(t) for t in l[-1:]] ]
    FIRSTLAST = [ lambda l: ["From : " + str(l[0]), "Until: " + str(l[-1])] if len(l) > 1 else [str(t) for t in l] ]
    ALL       = [ lambda l: [str(t) for t in l] ]

    def __init__(self, wrapper):
        self.fn = wrapper[0]
    def toAdoc(self, times):
        return " +\n       ".join(str(t) for t in self.fn(times))
    def toHtml(self, times):
        return "<br/>".join(str(t) for t in self.fn(times))

class Action(base.Action):
    _storage = config.Config().asMap().get("storage", "/var/lib/pyruse") \
        + "/" + os.path.basename(__file__) + ".journal"
    _out = None
    _hour = 0

    _txtDocStart = '= Pyruse Report\n\n'
    _txtHeadWarn = '== WARNING Messages\n\n'
    _txtHeadInfo = '\n== Information Messages\n\n'
    _txtHeadOther = '\n== Other log events\n\n'
    _txtTableDelim = '|===============================================================================\n'
    _txtTableHeader = '|Count|Message                                    |Date+time for each occurrence\n'
    _txtPreDelim = '----------\n'

    _htmDocStart = '<html>\n<head><meta charset="utf-8"/><style type="text/css">td{vertical-align: top}</style></head>\n<body>\n<h1>Pyruse Report</h1>\n'
    _htmDocStop = '</body></html>'
    _htmHeadWarn = '<h2>WARNING Messages</h2>\n'
    _htmHeadInfo = '<h2>Information Messages</h2>\n'
    _htmHeadOther = '<h2>Other log events</h2>\n'
    _htmTableStart = '<table border="1">\n<tr><th>Count</th><th>Message</th><th>Date+time for each occurrence</th></tr>\n'
    _htmTableStop = '</table>\n'
    _htmPreStart = '<pre>'
    _htmPreStop = '</pre>\n'

    def _closeJournal():
        Action._out.write("{}]")
        Action._out.close()
        Action._out = None

    def _openJournal():
        if Action._out is None:
            if os.path.exists(Action._storage):
                Action._out = open(Action._storage, "a", 1)
            else:
                Action._out = open(Action._storage, "w", 1)
                Action._out.write("[\n")

    def __init__(self, args):
        super().__init__()

        l = args["level"]
        if l == "WARN":
            self.level = 1
        elif l == "INFO":
            self.level = 2
        else:
            self.level = 0

        self.template = args["message"]
        values = {}
        for (_void, name, _void, _void) in string.Formatter().parse(self.template):
            if name:
                values[name] = None
        self.values = values

        ts = args.get("details", Details.ALL.name)
        for e in Details:
            if ts == e.name:
                self.details = e
                break
        else:
            self.details = Details.ALL

    def act(self, entry):
        for (name, _void) in self.values.items():
            self.values[name] = entry.get(name, None)
        msg = self.template.format_map(self.values)
        json.dump(
            OrderedDict(L = self.level, T = entry["__REALTIME_TIMESTAMP"].timestamp(), M = msg, D = self.details.name),
            Action._out
        )
        Action._out.write(",\n")
        thisHour = datetime.today().hour
        if thisHour < Action._hour:
            Action._closeJournal()
            self._sendReport()
            Action._openJournal()
        Action._hour = thisHour

    def _encode(self, text):
        return text.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;')

    def _toAdoc(self, msg, times):
        return "\n|{count:^5d}|{text}\n      |{times}\n".format_map({
            "count": sum(len(t) for (_void, t) in times.items()),
            "text": msg,
            "times": "\n       +\n       ".join(d.toAdoc(t) for (d,t) in times.items())
        })

    def _toHtml(self, msg, times):
        return "<tr><td>{count}</td><td>{text}</td><td>{times}</td></tr>\n".format_map({
            "count": sum(len(t) for (_void, t) in times.items()),
            "text": self._encode(msg),
            "times": "<br/><br/>".join(d.toHtml(t) for (d,t) in times.items())
        })

    def _sendReport(self):
        messages = [[], {}, {}]
        with open(Action._storage) as journal:
            for e in json.load(journal):
                if e != {}:
                    (L, T, M, D) = (e["L"], datetime.fromtimestamp(e["T"]), e["M"], e.get("D", Details.ALL.name))
                    if L == 0:
                        messages[0].append((T, M))
                    else:
                        dd = Details[D]
                        if M not in messages[L]:
                            messages[L][M] = {}
                        if dd not in messages[L][M]:
                            messages[L][M][dd] = []
                        messages[L][M][dd].append(T)
        os.remove(Action._storage)

        html = Action._htmDocStart + Action._htmHeadWarn
        text = Action._txtDocStart + Action._txtHeadWarn

        text += Action._txtTableDelim + Action._txtTableHeader
        html += Action._htmTableStart
        for (msg, times) in sorted(messages[1].items(), key = lambda i: i[0]):
            text += self._toAdoc(msg, times)
            html += self._toHtml(msg, times)
        text += Action._txtTableDelim
        html += Action._htmTableStop

        text += Action._txtHeadInfo
        html += Action._htmHeadInfo

        text += Action._txtTableDelim + Action._txtTableHeader
        html += Action._htmTableStart
        for (msg, times) in sorted(messages[2].items(), key = lambda i: i[0]):
            text += self._toAdoc(msg, times)
            html += self._toHtml(msg, times)
        text += Action._txtTableDelim
        html += Action._htmTableStop

        text += Action._txtHeadOther
        html += Action._htmHeadOther

        text += Action._txtPreDelim
        html += Action._htmPreStart
        for (time, msg) in messages[0]:
            m = '%s: %s\n' % (time, msg)
            text += m
            html += self._encode(m)
        text += Action._txtPreDelim
        html += Action._htmPreStop
        html += Action._htmDocStop

        email.Mail(text, html).send()

Action._openJournal()
