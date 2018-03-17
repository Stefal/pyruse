# Log entries creation, and ban of IP addresses

## Log entries

The main purpose of creating new log entries, is to detect recidives in bad behaviour: after an IP address misbehaves, it gets banned, and we generate a log line for that; such log lines get counted, and eventually trigger a harsher, recidive, ban of the same IP address. Several levels of bans can thus be stacked, up to an unlimited ban, if such is wanted.

Action `action_log` takes a mandatory `message` argument, which is a template for the message to be sent.
Optionally, the log level can be changed from the default (which is “INFO”) by setting the `level` parameter; valid values are “EMERG”, “ALERT”, “CRIT”, “ERR”, “WARNING”, “NOTICE”, “INFO”, and “DEBUG” (see [Syslog severity levels](https://en.wikipedia.org/wiki/Syslog#Severity_level) for the definitions).

The `message` parameter is a Python [string format](https://docs.python.org/3/library/string.html#formatstrings).
This means that any key in the current entry may be referrenced by its name between curly braces.
This also means that literal curly braces must be doubled, lest they are read as the start of a template placeholder.

Here are some examples:

```json
{
  "action": "action_log", "args": { "message": "Ban from SSH for {thatIP}." }
}

{
  "action": "action_log",
  "args": {
    "level": "NOTICE",
    "message": "Recidive ban from SSH for {thatIP}."
  }
}
```

## Ban IP addresses after they misbehaved

Linux provides a number of firewall solutions: [iptables](http://www.netfilter.org/), its successor [nftables](http://wiki.nftables.org/), and many iptables frontends like [Shorewall](http://www.shorewall.net/) or RedHat’s [firewalld](http://www.firewalld.org/).

For Pyruse, **nftables** was initially chosen, because it is modern and light-weight, and provides interesting features. Besides, only nftables is actually tested in real use.  
Now, an **ipset**-based [alternative is also available](#iptables-with-ipset).

### nftables

Action `action_nftBan` requires that nftables is installed.
In addition, the binary to run (and parameters if needed) should be set in the configuration file; here is the default value:

```json
"nftBan": {
  "nft": [ "/usr/bin/nft" ]
}
```

This action takes three mandatory arguments:

* `nftSetIPv4` and `nftSetIPv6` are the nftables sets where IP addresses will be added. The IP address is considered to be IPv6 if it contains at least one colon (`:`) character, else it is supposed to be IPv4.
* `IP` gives the name of the entry field to read, in order to get the IP address.

Note that nftables sets are kind-of paths.
For example, if your nftables ruleset is such:

```nftables
table ip Inet4 {
  set mail_ban {
    type ipv4_addr
    flags timeout
  }

  chain FilterIn {
    type filter hook input priority 0
    policy drop
    ip saddr @mail_ban drop
    …
  }
}
table ip6 Inet6 {
  set mail_ban {
    type ipv6_addr
    flags timeout
  }

  chain FilterIn {
    type filter hook input priority 0
    policy drop
    ip6 saddr @mail_ban drop
    …
  }
}
```

Then the values for `nftSetIPv4` and `nftSetIPv6` will be respectively “`ip Inet4 mail_ban`” and “`ip6 Inet6 mail_ban`”.

Optionally, a number may be specified with `banSeconds` to limit the time this ban will last.
The nice thing with nftables, is that it handles the timeouts itself: no need to keep track of the active bans and remove them using a Python program; the only reason why bans are recorded in a file, is to be able to restore them on reboot.

Here are examples:

```json
{
  "action": "action_nftBan",
  "args": { "IP": "thatIP", "banSeconds": 86400, "nftSetIPv4": "ip Inet4 mail_ban", "nftSetIPv6": "ip6 Inet6 mail_ban" }
}

{
  "action": "action_nftBan",
  "args": { "IP": "thatIP", "nftSetIPv4": "ip Inet4 sshd_ban", "nftSetIPv6": "ip6 Inet6 sshd_ban" }
}
```

#### List the currently banned addresses

To see what IP addresses are currently banned, here is the `nft` command:

```bash
$ sudo nft 'list set ip Inet4 mail_ban'
table ip Inet4 {
        set mail_ban {
                type ipv4_addr
                flags timeout
                elements = { 37.49.226.159 timeout 5d expires 3d11h11m58s,
                             71.6.167.142 timeout 2d20h23m57s expires 6h12m49s,
                             91.200.12.96 timeout 4d18h22m29s expires 2d4h11m24s,
                             91.200.12.156 timeout 4d22h59m19s expires 2d8h48m15s,
                             91.200.12.203 timeout 4d22h53m38s expires 2d8h42m33s,
                             91.200.12.213 timeout 4d22h54m25s expires 2d8h43m21s,
                             91.200.12.217 timeout 4d18h1m14s expires 2d3h50m9s,
                             91.200.12.230 timeout 4d22h54m27s expires 2d8h43m23s,
                             139.201.42.59 timeout 5d expires 3d17h29m51s,
                             183.129.89.243 timeout 5d expires 4d9h4m37s }
        }
}
```

_Note_: The un-rounded timeouts are post-reboot restored bans.

#### Un-ban an IP address

It is bound to happen some day: you will want to un-ban a banned IP address.

Since `action_nftBan` does not keep the current bans in memory, it is enough to remove the ban using the `nft` command:

```bash
$ sudo nft 'delete element ip Inet4 mail_ban {10.0.0.10}'
```

However, the ban may be restored when restarting Pyruse.
To avoid that, also delete the corresponding record from the `action_nftBan.py.json` file in Pyruse’s [storage directory](conffile.md).

To go further, you could tweak your configuration, so that your trusted IP addresses never reach `action_nftBan`.

#### Manual ban of an IP address

To add a ban yourself, run a command like this:

```bash
$ sudo nft 'add element ip Inet4 ssh_ban {192.168.1.1 timeout 5d}
```

The `timeout …` part can be omitted to add a permanent ban. The timeout can be any combination of days (`d`), hours (`h`), minutes (`m`), and seconds (`s`), eg. “`3d31m16s`”.

In order to make the ban persistent across reboots, a corresponding record should also be appended to the `action_nftBan.py.json` file in Pyruse’s [storage directory](conffile.md) (the IP address, Nftables Set, days, hours, minutes, seconds, and actual path to the file should be adapted to your needs):

* either a time-limited ban:

```bash
$ sudo sed -i "\$s/.\$/$(date +', {"IP": "192.168.1.1", "nfSet": "ip Inet4 ssh_ban", "timestamp": %s.000000}' -d 'now +3day +31minute +16second')]/" /var/lib/pyruse/action_nftBan.py.json
```

* or an unlimited ban:

```bash
$ sudo sed -i '$s/.$/, {"IP": "192.168.1.1", "nfSet": "ip Inet4 ssh_ban", "timestamp": 0}]/' /var/lib/pyruse/action_nftBan.py.json
```

### iptables with ipset

Action `action_ipsetBan` requires that ipset and iptables are installed.
In addition, the ipset binary to run (and parameters if needed) should be set in the configuration file; here is the default value:

```json
	"ipsetBan": {
		"ipset": [ "/usr/bin/ipset", "-exist", "-quiet" ]
	}
```

This action works exactly [like `action_nftBan`](#nftables), except parameters `nftSetIPv4` and `nftSetIPv6` are named `ipSetIPv4` and `ipSetIPv6` instead.

The name of the set in the `ipSetIPv4` parameter must have been created before running Pyruse, with:

```bash
$ sudo ipset create SET_NAME hash:ip family inet hashsize 1024 maxelem 65535
```

Likewise, the set given by `ipSetIPv6` must have been created before running Pyruse, with:

```bash
$ sudo ipset create SET_NAME hash:ip family inet6 hashsize 1024 maxelem 65535
```

Here are examples of usage for this action:

```json
{
  "action": "action_ipsetBan",
  "args": { "IP": "thatIP", "banSeconds": 86400, "ipSetIPv4": "mail_ban4", "ipSetIPv6": "mail_ban6" }
}

{
  "action": "action_ipsetBan",
  "args": { "IP": "thatIP", "ipSetIPv4": "sshd_ban4", "ipSetIPv6": "sshd_ban6" }
}
```

#### List the currently banned addresses

To see what IP addresses are currently banned, here is the `ipset` command:

```bash
$ sudo ipset list mail_ban4'
```

#### Un-ban an IP address

To remove an IP address from a set, here is the `ipset` command:

```bash
$ sudo ipset del mail_ban4 10.0.0.10'
```

To make the change persistent across reboots, also delete the corresponding record from the `action_ipsetBan.py.json` file in Pyruse’s [storage directory](conffile.md).

To go further, you could tweak your configuration, so that your trusted IP addresses never reach `action_ipsetBan`.

#### Manual ban of an IP address

To add a ban yourself, run a command like this:

```bash
$ sudo ipset add ssh_ban4 192.168.1.1 timeout 261076
```

The `timeout …` part can be omitted to add a permanent ban; otherwise it is a number of seconds.

In order to make the ban persistent across reboots, a corresponding record should also be appended to the `action_ipsetBan.py.json` file in Pyruse’s [storage directory](conffile.md) (the IP address, Nftables Set, days, hours, minutes, seconds, and actual path to the file should be adapted to your needs):

* either a time-limited ban:

```bash
$ sudo sed -i "\$s/.\$/$(date +', {"IP": "192.168.1.1", "nfSet": "ssh_ban4", "timestamp": %s.000000}' -d 'now +3day +31minute +16second')]/" /var/lib/pyruse/action_ipsetBan.py.json
```

* or an unlimited ban:

```bash
$ sudo sed -i '$s/.$/, {"IP": "192.168.1.1", "nfSet": "ssh_ban4", "timestamp": 0}]/' /var/lib/pyruse/action_ipsetBan.py.json
```
