# The daily report

This module fulfills one of the initial goal of the project: recording significant or unexpected log entries, and sending a report by email at the end of each day.

Currently, this module generates emails in both HTML and text ([asciidoc](http://asciidoctor.org/docs/asciidoc-syntax-quick-reference/)), with a static layout.
The layout goes roughtly like this:

```asciidoc
= Main title

== Title of the “WARN” section

… a table with the warnings of the past day …

== Title of the “INFO” section

… a table with the notable facts of the past day …

== Title of the “OTHER” section

… a list of the unexpected log entries for the past day …
```

The intended usage of Pyruse is to write the configuration so that:

1. When a log entry requires immediate attention, an email is sent; the fact may also be put in the report, for example in the `INFO` section (logically, the issue has already been taken care of when the daily report arrives).
2. If the issue is less urgent but still suspect or important, and should be reviewed, it is put in the `WARN` section of the report.
3. Other significant facts go to the `INFO` section; depending on later observations, these facts may evolve to the `WARN` section or get dismissed, by altering the configuration.
4. All known uninteresting log entries are discarded. The last execution chain would be an `action_dailyReport` for the `OTHER` section, that would record all log entries that went through all previous execution chains without being handled nor being discarded: these are the unexpected log entries, or log entries that are known but are too rare to be granted dedicated configuration rules.

When an `action_dailyReport` is used, there are two mandatory parameters:

* the `level` must be one of `WARN`, `INFO`, or `OTHER`;
* the `message` is a Python [string format](https://docs.python.org/3/library/string.html#formatstrings):
    - this means that any key in the current entry may be referrenced by its name between curly braces;
    - and that literal curly braces must be doubled, lest they are read as the start of a template placeholder.

Additionally, the `details` parameter may be used to fine-tune the rendering of the times at which events occur (see below).

In the `WARN` and `INFO` sections, there is one table row per unique message, and the messages are sorted in alphabetical order.
On each row, the table cells contain first the number of times the message was added to the section, then the message itself, and finally the dates and times of occurrence:

* If `details` is “`ALL`” or unspecified, each occurrence is mentionned.
* If `details` is “`NONE`”, no occurrence is mentionned.
* If `details` is “`FIRST`”, only the first occurrence is mentionned, prepended by “`From :`”.
* If `details` is “`LAST`”, only the last occurrence is mentionned, prepended by “`Until:`”.
* “`FIRSTLAST`” is a combination of “`FIRST`” and “`LAST`”, although it falls back to “`ALL`” when there are fewer than 2 occurrences.

_Notes_:
* As a consequence, it is useless to put the date and time of occurrence in the message.
* If the same message is added to a section with different levels of details, each level of details gets its own paragraph in the third column.

In the `OTHER` section, the messages are kept in chronological order, and prepended by their date and time of occurrence: “`date+time: message`”. It is thus useless to put the date and time of occurrence in the message.

Here are examples for each of the sections:

```json
{
  "action": "action_dailyReport",
  "args": { "level": "WARN", "message": "Nextcloud query failed because the buffer-size was too low", "details": "NONE" }
}

{
  "action": "action_dailyReport",
  "args": { "level": "INFO", "message": "XMPP server {xmppServer} did not provide a secure connection" }
}

{
  "action": "action_dailyReport",
  "args": { "level": "OTHER", "message": "[{PRIORITY}/{_HOSTNAME}/{_SYSTEMD_UNIT}] {MESSAGE}" }
}
```

I chose the `WARN` level for the first situation because, although there is no immediate security risk associated with this fact, I know that some users will experience a loss of functionality. However, the exact times of occurrence are of no use; this is just a situation I need to be aware of.

I chose the `INFO` level for the second situation because all is well with my server; however, depending on who the remote `xmppServer` is, I might want to add it to a whitelist of allowed unsecured peers.

As for the last example, it is the catch-all action, that will report unexpected log lines.

_Tip_: System administrators should know that the contents of the next daily report can always be read in Pyruse’s [storage directory](conffile.md), in the file named `action_dailyReport.py.journal`.
In this file, `L` is the section (aka. level: `1` for `WARN`, `2` for `INFO`, and `0` for `OTHER`), `T` is the Unix timestamp, `M` is the message, and `D` is the level of details regarding the times of occurrence.
