# General structure of `pyruse.json` file

## Syntax

The Pyruse configuration file is in [JSON format](http://json.org/).
In short:

* There are dictionaries, where keys of type `string` are mapped to values of any type, using this syntax:

```json
{
  "1st key": "value1",
  "2nd key": "value2",
  "last key": "no trailing comma"
}
```

* Lists are similar, with comma-separated items between square-brackets:

```json
[ 1, 10, 100 ]
```

* Values in dictionaries and lists can be:
    - Unicode character strings, enclosed in straight double-quotes;
    - all-digit numbers, with an optional dot if a decimal separator is needed;
    - or the `true`, `false`, and `null` keywords, that have the same meaning as `True`, `False`, and `None` in Python.
* Inside strings, literal straight double-quotes (`"`) and back-slashes (`\`) must be prepended with a `\`. Besides, a `\` may be placed before some characters to alter their meaning, from a simple letter to a control character:
    - `b`, `f`, `n`, `r`, `t` translate to backspace, form-feed, newline, carriage-return, and horizontal tabulation;
    - `u` followed by 4 hexadecimal digits allows to enter hard-to-type Unicode characters using their codes.

The whole configuration file is a dictionary, where the core of Pyruse as well as some modules expect configuration entries.

## Core entries

The main entry in the configuration dictionary is the **`"actions"`** key.
Its value is a dictionary of execution chains, where each chain is identified by its label (a key in the `"actions"` dictionary).
Each chain is a list of filters and actions, described below.

Another core entry in the configuration dictionary is the **`"email"`** key.
Its value is a dictionary containing:

* `"from"` (string): the email address that will appear as sender of emails sent by Pyruse;
* `"to"` (list): the email addresses of the people to whom Pyruse emails will be sent;
* `"subject"` (string): the default subject of Pyruse emails;
* `"sendmail"` (list): the binary that will send emails (it is the first item in the list; usually sendmail, which may be a link to exim, msmtp, etc.), then optionally other strings that will be used as parameters of this binary; this program will be launched once for each email, and receive them on the standard input.

Also important are the **`"storage"`** key, a string that gives the path where Pyruse core and its modules are allowed to write state data (usually under `/var/lib`);
and the **`"8bit-message-encoding"`** key, a string that gives an 8-bit encoding to be used in cases when the systemd message is not valid UTF-8 (for example `iso-8859-1`).

Finally, the **`"debug"`** key (boolean), when set to `true`, lets Pyruse remember the action-chains’ names while reading the configuration, which results in better information when Pyruse encounters a problem.

### Actions and filters in execution chains

In an execution chain, both filters and chains are written as dictionaries. For filters, the `"filter"` key gives the module basename of the filter to use; likewise for actions, the `"action"` key gives the module basename of the action to use.
Both filters and actions also use the `"args"` key to give parameters to the module (it is a dictionary).

Besides, both filters and actions implicitely use a `"then"` key; the former also implicitely use an `"else"` key:

* `"then"` is automatically linked to the next module in the chain; if there is none:
    - filters then pass control to the next available execution chain,
    - while actions just stop there (no further handling of the log entry).
* `"else"` is only for filters, and acts the same as `"then"` does, except it is used when the filter does not pass, whereas `"then"` is used when the filter does pass.

It should be noted, that if an error happens inside an action, this action simply stops and no more processing happens for the current log entry; if an error happens inside a filter, this filter is considered non-passing, and the rules that apply are those of the `"else"` link.

If the default linking does not achieve the wanted result, it can be overriden, by explicitely giving the wanted execution chain’s name as a value of `"then"` or `"else"`.
_Important_: The configuration file is read from top to bottom. When an execution chain that has already been used as the target of an explicit `"then"` or `"else"` is encountered, this chain is skipped where default linking is concerned. This allows to write “jump chains” (as in Netfilter). Note that such chains should be declared below the location where they are first called, not above, or the result may not be what is expected.

See the documentation associated with specific filters and actions to get details on their expectations regarding the `"args"` dictionary.

## Module entries

By convention, a module that needs its own configuration entries puts them in a dictionary, introduced by a key reflecting the module’s name.

See the documentation associated with specific filters and actions to get details on their specific configuration needs.

## Example configuration file

Here is a minimal example:

```json
{
  "actions": {
    "Immediately warn of fatal errors": [
      {
        "filter": "filter_lowerOrEquals",
        "args": { "field": "PRIORITY", "value": 1 }
      },
      {
        "action": "action_email",
        "args": { "subject": "Pyruse", "message": "Error on {_HOSTNAME} on {__REALTIME_TIMESTAMP}:\n{MESSAGE}" }
      }
    ],
    "Discard info entries": [
      {
        "filter": "filter_greaterOrEquals",
        "args": { "field": "PRIORITY", "value": 4 }
      },
      {
        "action": "action_noop"
      }
    ],
    "Report everything else": [
      {
        "action": "action_dailyReport",
        "args": { "level": "OTHER", "message": "[{PRIORITY}] {_SYSTEMD_UNIT}/{_HOSTNAME}: {MESSAGE}" }
      }
    ]
  },
  "email": {
    "from": "pyruse@example.org",
    "to": [ "hostmaster@example.org" ],
    "subject": "Pyruse Report",
    "sendmail": [ "/usr/bin/sendmail", "-t" ]
  },
  "8bit-message-encoding": "iso-8859-1",
  "storage": "/var/lib/pyruse",
  "debug": false
}
```
