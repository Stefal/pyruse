# DNAT-correcting actions

## Introduction

Pyruse provides two actions, namely `action_dnatCapture` and `action_dnatReplace`, that work together towards a single goal: giving to Pyruse’s filters and actions the illusion that client connections come directly to the examined services, instead of going through a firewall Network Address Translation or a reverse-proxy.

If for example you run a Prosody XMPP server (or anything that does not handle the [PROXY protocol](https://www.haproxy.org/download/1.8/doc/proxy-protocol.txt)) behind an HAProxy reverse-proxy, or if all your network connections go through a NAT’ing router in your DMZ, then the following happens: the services report your proxy as being the client, ie. usually `127.0.0.1` (same machine) or your LAN’s router IP.

Here is a simplified illustration of the network configuration:

```ditaa
/-------------------------------\
|         +---------------------+
| Client  | ClientIP:ClientPort +---\==\
|         +---------------------+   |  :
\-------------------------------/   |  :
                                 (1)|  :(2)
/-------------------------------\   |  :
|       +--++-------------------+   |  :
|       |  PublicIP:PublicPort  +<--/  :
|       +-----------------------+      :    /---------------------------------\
| Proxy                         |      :    +-----------------------+         |
|       +-----------------------+      \===>+                       | Service |
|       | ProxyLanIP:RandomPort +---------->+ ServiceIP:ServicePort |         |
|       +-----------------------+   (1)     +-----------------------+         |
\-------------------------------/           \---------------------------------/
```

The circuit number `(1)` is the real one, which is why the service sees `ProxyLanIP:RandomPort` as the client.
The circuit number `(2)` is what Pyruse will fake, using the fore-mentionned actions.

First some “vocabulary”. In `action_dnatCapture` and `action_dnatReplace`:

* `ClientIP` and `ClientPort` are called `saddr` and `sport` (`s` for **s**ource)
* `ProxyLanIP` and `RandomPort` are called `addr` and `port`
* `ServiceIP` and `ServicePort` are called `daddr` and `dport` (`d` for **d**estination)

Pyruse’s actions work by storing the link between these 6 values in memory, and later replacing `addr` with `saddr`, and optionaly `port` with `sport`.

## Action `action_dnatCapture`

For Pyruse to be able to capture the wanted values, the proxy software must first be configured to provide them.

### HAProxy configuration

Here is an example configuration that reproduces the default `tcplog` format, simply adding the missing information between square braquets:

```haproxy
global
    log        /dev/log local0 info
    … misc. other options …

defaults
    mode       tcp
    log        global
    option     log-separate-errors
    log-format "%ci:%cp [%t] %ft %b[%bi:%bp]/%s %Tw/%Tc/%Tt %B %ts %ac/%fc/%bc/%sc/%rc %sq/%bq"
    … misc. other options …
```

The above configuration would produce log lines like this one:

```log
12.34.56.78:54321 [dd/MM/yyyy:HH:mm:ss.…] tls~ xmpp[10.0.0.1:43210]/xmpp …/…/… … -- …/…/…/…/… …/…
```

### nftables configuration

Here is an example rule for nftables on the proxy:

```nftables
tcp dport 22 log prefix "DNAT/ssh: " dnat to 10.0.0.2
```

Having an easily recognizable log prefix helps.
The above would result in a line like this one:

```log
DNAT/ssh: IN=… OUT=… MAC=… SRC=12.34.56.78 DST=10.0.0.1 LEN=… … PROTO=… SPT=43210 DPT=22 WINDOW=… …
```

Besides, Netfilter logs (part of the kernel logs) must be enabled for those log lines to actually appear in the logs.
For example, it [may be required](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=2851940ffee313e0ff12540a8e11a8c54dea9c65) to run this:

```bash
sysctl net.netfilter.nf_log_all_netns=1
```

### Pyruse configuration

Action `action_dnatCapture` must tell Pyruse what fields in the current log entry match the 6 parameters described in the introduction.
If some values are constant and known (or marker values are wished for), but these values are in no available field in the log entry, then those values may be given in the parameters `addrValue`, `portValue`, `daddrValue`, and `dportValue`.
When an information is given by both a field reference and a plain value, the field reference is used first, and the plain value is used as a default value if the referenced field is not found.

The `saddr` parameter is mandatory (else there is no point), and either `addr` or `addrValue` (or both) must be given as well.

In addition to all these parameters, a `keepSeconds` parameter may be given to indicate how many seconds the detected correspondance should be kept in memory (default value is 63).

NOTE: For performance reasons, the `keepSeconds` value is rounded up to the _next_ power of 2 — eg. values 4 to 7 are rounded up to 8 —, and the retention countdown only begins at the next occurrence of that power of two in the current time, expressed as a Unix timestamp.  
As a consequence, the actual length of time a correspondance is kept in memory, varies between 1× and 4× the length of time given by the parameter, depending on the chosen value, and depending on the current time-of-the-day when the correspondance is found.

Here is an example configuration that would work fine with log lines as produced by nftables (see above):

```json
{ "filter": "filter_pcre",
  "args": {
    "field": "MESSAGE",
    "re": "^DNAT/ssh:.* SRC=([^ ]+) DST=([^ ]+) .* SPT=([^ ]+) DPT=([^ ]+) ",
    "save": [ "dnatSaddr", "dnatAddr", "dnatPort", "dnatDport" ]
  }
},
{ "action": "action_dnatCapture",
  "args": {
    "saddr": "dnatSaddr",
    "addr":  "dnatAddr",  "addrValue":  "127.0.0.1",
    "port":  "dnatPort",
    "dport": "dnatDport", "dportValue": "22"
  }
}
```

## Action `action_dnatReplace`

Action `action_dnatReplace` should be inserted whenever there is a chance that the values stored in a log entry’s fields for a client IP address (and possibly the port as well) are those of a proxy instead.

Properties `addr`, `port`, `daddr`, and `dport` are used to match against a correspondance currently held in memory; at least one of these properties must be given.
Each property corresponds to the name of a log entry field in which to read the corresponding value.

Properties `saddrInto` and `sportInto` indicate the log entry fields in which to store the corrected source IP address or port; at least one of those properties must be given.

For example, consider the following (simplified) log entries:

```json
{ '_HOSTNAME': 'dmz',
  'SYSLOG_IDENTIFIER': 'kernel',
  'MESSAGE': 'DNAT/ssh: … SRC=12.34.56.78 DST=10.0.0.1 … SPT=43210 …'
}
{ '_HOSTNAME': 'sshserv',
  'SYSLOG_IDENTIFIER': 'sshd',
  '_SYSTEMD_UNIT': 'sshd.service',
  'MESSAGE': 'Failed password for ME from 10.0.0.1 port 43210 ssh2'
}
{ '_HOSTNAME': 'dmz',
  'SYSLOG_IDENTIFIER': 'sshd',
  '_SYSTEMD_UNIT': 'sshd.service',
  'MESSAGE': 'Failed password for KeyUser from 87.65.43.21 port 24680 ssh2'
}
```

Assuming the first log entry is correctly handled by `action_dnatCapture`, a good configuration to handle SSH failed logins could be:

```json
{ "filter": "filter_equals",
  "args": { "field": "SYSLOG_IDENTIFIER", "value": "sshd" }
},
{ "filter": "filter_pcre",
  "args": {
    "field": "MESSAGE",
    "re": "^Failed password for (.*) from ([^ ]+) port ([^ ]+) ssh2$",
    "save": [ "sshUser", "clientIP", "clientPort" ]
},
{ "action": "action_dnatReplace",
  "args": { "addr": "clientIP", "port": "clientPort", "saddrInto": "clientIP" }
},
{ "action": "action_email",
  "args": { "message": "SSH attack from {clientIP} on {sshUser}@{_HOSTNAME}." }
}
```

As a result, two emails would be sent, with these messages:

```
SSH attack from 12.34.56.78 on ME@sshserv.
SSH attack from 87.65.43.21 on KeyUser@dmz.
```
