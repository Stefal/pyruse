# Ban IP addresses after they misbehaved

Linux provides a number of firewall solutions: [iptables](http://www.netfilter.org/), its successor [nftables](http://wiki.nftables.org/), and many iptables frontends like [Shorewall](http://www.shorewall.net/) or RedHat’s [firewalld](http://www.firewalld.org/).
For Pyruse, **nftables** was chosen, because it is modern and light-weight, and provides interesting features.

Action `action_nftBan` takes three mandatory arguments:

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

Then the values for `nftSetIPv4` and `nftSetIPv6` will be respectively “`Inet4 mail_ban`” and “`Inet6 mail_ban`”.

Optionally, a number may be specified with `banSeconds` to limit the time this ban will last.
The nice thing with nftables, is that it handles the timeouts itself: no need to keep track of the active bans and remove them using a Python program; the only reason why bans are recorded in a file, is to be able to restore them on reboot.

Here are examples:

```json
{
  "action": "action_nftBan",
  "args": { "IP": "thatIP", "banSeconds": 86400, "nftSetIPv4": "Inet4 mail_ban", "nftSetIPv6": "Inet6 mail_ban" }
}

{
  "action": "action_nftBan",
  "args": { "IP": "thatIP", "nftSetIPv4": "Inet4 sshd_ban", "nftSetIPv6": "Inet6 sshd_ban" }
}
```

## List the currently banned addresses

To see what IP addresses are currently banned, here is the `nft` command:

```bash
$ sudo nft 'list set Inet4 mail_ban'
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

## Un-ban an IP address

It is bound to happen some day: you will want to un-ban a banned IP address.

Since `action_nftBan` does not keep the current bans in memory, it is enough to remove the ban using the `nft` command:

```bash
$ sudo nft 'delete element Inet4 mail_ban {10.0.0.10}'
```

However, the ban may be restored when restarting Pyruse.
To avoid that, also delete the corresponding record from the `action_nftBan.py.json` file in Pyruse’s [storage directory](conffile.md).

To go further, you could tweak your configuration, so that your trusted IP addresses never reach `action_nftBan`.
