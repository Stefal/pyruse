import string
from pyruse import base, email

class Action(base.Action):
    def __init__(self, args):
        super().__init__()
        self.subject = args.get("subject", "Pyruse Notification")
        self.template = args["message"]
        values = {}
        for (_void, name, _void, _void) in string.Formatter().parse(self.template):
            if name:
                values[name] = None
        self.values = values

    def act(self, entry):
        for (name, _void) in self.values.items():
            self.values[name] = entry.get(name, None)
        msg = self.template.format_map(self.values)
        email.Mail(msg).setSubject(self.subject).send()
