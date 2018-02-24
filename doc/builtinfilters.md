# Built-in filters

Pyruse comes with a few very simple filters.

## `=`, `≤`, `≥`, `in`

The filters `filter_equals`, `filter_lowerOrEquals`, and `filter_greaterOrEquals` simply check equality or inequality between a given field, given by the parameter `field`, and a constant value, given py the parameter `value`.
Both parameters are mandatory.
Here are two examples:

```json
{
  "filter": "filter_greaterOrEquals",
  "args": { "field": "IPfailures", "value": 6 }
}

{
  "filter": "filter_equals",
  "args": { "field": "_SYSTEMD_UNIT", "value": "nginx.service" }
}
```

Filter `filter_in` works the same way as `filter_equals` does, except that instead of a single `value`, a `values` list is given, and equality between the field’s contents and any of the list’s items is considered a success.
Here is an example:

```json
{
  "filter": "filter_in",
  "args": { "field": "PRIORITY", "values": [ 2, 3 ] }
}
```

For any of these filters, the constant values must be of the same type as the typical contents of the chosen field.

## Test if an IP address is part of given networks

Filter `filter_inNetworks` reads an IP address in a field given by the `field` parameter, and a list of networks in the `nets` parameter; each net is written as an IP address, then “`/`”, then an integer network mask.

The filter is passing if the IP address that was read is part of one of the networks configured for the filter.

Here is an example:

```json
{
  "filter": "filter_inNetworks",
  "args": { "field": "IP", "nets": [ "fd00::/8", "10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16" ] }
}
```

## Perl-compatible regular expressions (pcre)

Filter `filter_pcre` should only be used on character strings.
Like the above filters, it works on a field given by the `field` parameter, and the [regular expression](https://docs.python.org/3/library/re.html) being looked for is given by the `re` parameter.
Both parameters are mandatory.

The regular expression in the `re` parameter may contain capturing groups:

* Named capturing groups use the `(?P<groupName>…)` notation; the captured value is always stored under the key `groupName` in the current entry.
* Anonymous capturing groups stem from the use of simple parenthesis: `(…)`; these are not saved by default, but a `save` parameter (a list) may be specified, so that the captured values get stored in the current entry, using the names given by `save`.

Here are two identical examples:

```json
{
  "filter": "filter_pcre",
  "args": {
    "field": "MESSAGE",
    "re": "^\\{core\\} Login failed: '(.*)' \\(Remote IP: '(.*)'\\)",
    "save": [ "thatUser", "thatIP" ]
  }
}

{
  "filter": "filter_pcre",
  "args": {
    "field": "MESSAGE",
    "re": "^\\{core\\} Login failed: '(?P<thatUser>.*)' \\(Remote IP: '(?P<thatIP>.*)'\\)"
  }
}
```

Filter `filter_pcreAny` is to `filter_pcre` what `filter_in` is to `filter_equals`.
It works the same way as `filter_pcre`, except that instead of a single regular expression, its `re` parameter contains a list of regular expressions, and a match in the field’s contents is accepted with any of these regular expressions.

In contrast with `filter_pcre`, `filter_pcreAny` does not accept the `save` parameter: the order of fields cannot be guaranted to be the same accross several regular expressions.

Here is an example:

```json
{
  "filter": "filter_pcreAny",
  "args": {
    "field": "MESSAGE",
    "re": [
      "^Failed password for (?P<thatUser>.*) from (?P<thatIP>(?!192\\.168\\.1\\.201 )[^ ]*) port",
      "^Invalid user (?P<thatUser>.*) from (?P<thatIP>(?!192\\.168\\.1\\.201 )[^ ]*) port"
    ]
  }
}
```

## User existence

Filter `filter_userExists` knows of only one —mandatory— parameter: `field`.
This filter is passing, if the system reports the user whose name is the value of the chosen field [as existing](https://docs.python.org/3/library/pwd.html#pwd.getpwnam), and non-passing otherwise.

Here is an example:

```json
{
  "filter": "filter_userExists",
  "args": { "field": "thatUser" }
}
```
