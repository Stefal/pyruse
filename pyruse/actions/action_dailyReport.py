import datetime
import string
from pyruse import base, email

class Action(base.Action):
    WARN = "WARN"
    INFO = "INFO"
    OTHER = "OTHER"

    _messages = None
    _hour = 0

    _txtDocStart = '= Pyruse Report\n\n'
    _txtHeadWarn = '== WARNING Messages\n\n'
    _txtHeadInfo = '\n== Information Messages\n\n'
    _txtHeadOther = '\n== Other log events\n\n'
    _txtTableDelim = '|===============================================================================\n'
    _txtTableHeader = '|Count|Message                                    |Date+time for each occurrence\n'
    _txtPreDelim = '----------\n'

    _htmDocStart = '<html>\n<head><meta charset="utf-8"/></head>\n<body>\n<h1>Pyruse Report</h1>\n'
    _htmDocStop = '</body></html>'
    _htmHeadWarn = '<h2>WARNING Messages</h2>\n'
    _htmHeadInfo = '<h2>Information Messages</h2>\n'
    _htmHeadOther = '<h2>Other log events</h2>\n'
    _htmTableStart = '<table>\n<tr><th>Count</th><th>Message</th><th>Date+time for each occurrence</th></tr>\n'
    _htmTableStop = '</table>\n'
    _htmPreStart = '<pre>'
    _htmPreStop = '</pre>\n'

    def _encode(self, text):
        return text.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;')

    def _toAdoc(self, msg, times):
        return "\n|{count:^5d}|{text}\n      |{times}\n".format_map(
            {"count": len(times), "text": msg, "times": " +\n       ".join(str(t) for t in times)}
        )

    def _toHtml(self, msg, times):
        return "<tr><td>{count}</td><td>{text}</td><td>{times}</td></tr>\n".format_map(
            {"count": len(times), "text": self._encode(msg), "times": "<br/>".join(str(t) for t in times)}
        )

    def __init__(self, args):
        super().__init__()
        self.level = args["level"]
        self.isOther = self.level == Action.OTHER
        self.template = args["message"]
        values = {}
        for (_void, name, _void, _void) in string.Formatter().parse(self.template):
            if name:
                values[name] = None
        self.values = values

    def act(self, entry):
        messages = Action._messages[self.level]
        for (name, _void) in self.values.items():
            self.values[name] = entry.get(name, None)
        msg = self.template.format_map(self.values)
        if self.isOther:
            messages.append((entry["__REALTIME_TIMESTAMP"], msg))
        elif msg in messages:
            messages[msg].append(entry["__REALTIME_TIMESTAMP"])
        else:
            messages[msg] = [entry["__REALTIME_TIMESTAMP"]]
        thisHour = datetime.datetime.today().hour
        if thisHour < Action._hour:
            self._sendReport()
        Action._hour = thisHour

    def _sendReport(self):
        html = Action._htmDocStart + Action._htmHeadWarn
        text = Action._txtDocStart + Action._txtHeadWarn

        text += Action._txtTableDelim + Action._txtTableHeader
        html += Action._htmTableStart
        for (msg, times) in sorted(Action._messages[Action.WARN].items(), key = lambda i: i[0]):
            text += self._toAdoc(msg, times)
            html += self._toHtml(msg, times)
        text += Action._txtTableDelim
        html += Action._htmTableStop

        text += Action._txtHeadInfo
        html += Action._htmHeadInfo

        text += Action._txtTableDelim + Action._txtTableHeader
        html += Action._htmTableStart
        for (msg, times) in sorted(Action._messages[Action.INFO].items(), key = lambda i: i[0]):
            text += self._toAdoc(msg, times)
            html += self._toHtml(msg, times)
        text += Action._txtTableDelim
        html += Action._htmTableStop

        text += Action._txtHeadOther
        html += Action._htmHeadOther

        text += Action._txtPreDelim
        html += Action._htmPreStart
        for (time, msg) in Action._messages[Action.OTHER]:
            m = '%s: %s\n' % (time, msg)
            text += m
            html += self._encode(m)
        text += Action._txtPreDelim
        html += Action._htmPreStop
        html += Action._htmDocStop

        email.Mail(text, html).send()
        Action._messages = _resetMessages()

def _resetMessages():
    return {Action.WARN: {}, Action.INFO: {}, Action.OTHER: []}

Action._messages = _resetMessages()
