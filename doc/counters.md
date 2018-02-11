# Counter-based actions

Pyruse currently allows to raise a counter, with `action_counterRaise`, or to reset a counter (to zero), with `action_counterReset`. Adding an `action_counterLower` would be trivial, but has not been necessary so far.

Counters are kept in memory, by category; there are as many categories as wanted per the configuration file.
In each category, independent counters are kept for the different keys encountered while reviewing the log entries.
For example, there may be categories “mailFailures”, “sshFailures”, and “sshRecidives”, and then in each category there would be one counter per IP address that failed to use the service properly.

Thus, all counter-based actions need two mandatory parameters: the `counter` parameter gives the category name, and the `for` parameter indicates the field of the current entry in which the counter key must be read.

Besides, all counter-based actions accept the optional `save` parameter, which gives the name under which the resulting value of the counter should be stored in the current entry, for further processing (_note_: the value of a counter after being processed by `action_counterReset` is always `0`).
Finally:

* `action_counterRaise` may be given the `keepSeconds` parameter to specify how long this counter-raise should be recorded (indefinitely by default);
* `action_counterReset` may be given the `graceSeconds` parameter to specify how long this counter-reset should be enforced (the default is to immediately allow counter-raises).

Here are some examples:

```json
{
  "action": "action_counterRaise",
  "args": { "counter": "http", "for": "thatIP", "keepSeconds": 300, "save": "IPfailures" }
}

{
  "action": "action_counterRaise",
  "args": { "counter": "ssh", "for": "keyUser" }
}

{
  "action": "action_counterReset",
  "args": { "counter": "mail", "for": "emailSender", "graceSeconds": 900 }
}
```

Counters are auto-cleaned: they disapear when their value becomes zero (either with a reset, or due to `keepSeconds`), and they have no `graceSeconds` left.
If you use unlimited counters (no `keepSeconds`), be sure to reset them when you act on them after they have crossed a chosen threshold, so these counters can be “garbage-collected”.
