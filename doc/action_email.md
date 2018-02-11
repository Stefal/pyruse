# Send an email

Action `action_email`’s purpose is to send a text-only email.
To this end, the [core email settings](conffile.md) must be set.
The only mandatory parameter for this action is `message`, which is a template for the message to be sent.
Optionally, the subject can be changed from the default (which is “Pyruse Notification”) by setting the `subject` parameter, a simple string.

The `message` parameter is a Python [string format](https://docs.python.org/3/library/string.html#formatstrings).
This means that any key in the current entry may be referrenced by its name between curly braces.
This also means that literal curly braces must be doubled, lest they are read as the start of a template placeholder.

Here are some examples:

```json
{
  "action": "action_email",
  "args": { "message": "Error on {_HOSTNAME} on {__REALTIME_TIMESTAMP}:\n{MESSAGE}" }
}

{
  "action": "action_email",
  "args": {
    "subject": "Failure notification",
    "message": "[{numberOfFailures:^9d}] Failed login from {thatIP}."
  }
}
```

This last example renders as a centered space-padded-to-9-characters number between square brackets followed by a message, for example:

```
[   12    ] Failed login from 12.34.56.78.
```
