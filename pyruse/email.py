# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
import subprocess
from email.headerregistry import Address
from email.message import EmailMessage
from pyruse import config

class Mail:
	_mailConf = config.Config().asMap().get("email", {})

	def __init__(self, text, html = None):
		self.text = text
		self.html = html
		self.mailSubject = Mail._mailConf.get("subject", "Pyruse Report")
		self.mailFrom = Mail._mailConf.get("from", "pyruse")
		self.mailTo = Mail._mailConf.get("to", ["hostmaster"])

	def setSubject(self, subject):
		if subject:
			self.mailSubject = subject
		return self

	def send(self):
		message = EmailMessage()
		message["Subject"] = self.mailSubject
		message["From"] = Address(addr_spec = self.mailFrom)
		message["To"] = (Address(addr_spec = a) for a in self.mailTo)

		message.set_content(self.text)
		if self.html:
			message.add_alternative(self.html, subtype = "html")

		subprocess.run(
			Mail._mailConf.get("sendmail", ["/usr/bin/sendmail", "-t"]),
			input = message.as_bytes()
		)
