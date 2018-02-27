# pyruse is intended as a replacement to both fail2ban and epylog
# Copyright © 2017–2018 Y. Gablin
# Full licensing information in the LICENSE file, or gnu.org/licences/gpl-3.0.txt if the file is missing.
from pyruse.filters.filter_inNetworks import Filter

def whenIp4InNet4ThenTrue():
    assert Filter({"field": "ip", "nets": ["34.56.78.90/12"]}).filter({"ip": "34.48.0.1"})

def whenIp4NotInNet4ThenFalse():
    assert not Filter({"field": "ip", "nets": ["34.56.78.90/12"]}).filter({"ip": "34.47.255.254"})

def whenIp4ItselfThenTrue():
    assert Filter({"field": "ip", "nets": ["12.34.56.78"]}).filter({"ip": "12.34.56.78"})

def whenIp6InNet6ThenTrue():
    assert Filter({"field": "ip", "nets": ["2001:db8:1:1a0::/59"]}).filter({"ip": "2001:db8:1:1a0::1"})

def whenIp6NotInNet6ThenFalse():
    assert not Filter({"field": "ip", "nets": ["2001:db8:1:1a0::/59"]}).filter({"ip": "2001:db8:1:19f:ffff:ffff:ffff:fffe"})

def whenIp6ItselfThenTrue():
    assert Filter({"field": "ip", "nets": ["2001:db8:1:1a0::"]}).filter({"ip": "2001:db8:1:1a0::"})

def whenNumericIp6InNet4ThenFalse():
    assert not Filter({"field": "ip", "nets": ["34.56.78.90/12"]}).filter({"ip": "::2230:1"})

def whenNumericIp4InNet6ThenFalse():
    assert not Filter({"field": "ip", "nets": ["::2230:1/108"]}).filter({"ip": "34.48.0.1"})

def whenIpInOneNetworkThenTrue():
    assert Filter({"field": "ip", "nets": ["::2230:1/108", "10.0.0.0/8", "34.56.78.90/12", "2001:db8:1:1a0::/59"]}).filter({"ip": "34.48.0.1"})

def whenNoIpThenFalse():
    assert not Filter({"field": "ip", "nets": ["::2230:1/108", "10.0.0.0/8"]}).filter({"no_ip": "Hi!"})

def whenNoNetworkThenFalse():
    assert not Filter({"field": "ip", "nets": []}).filter({"ip": "34.48.0.1"})

def unitTests():
    whenIp4InNet4ThenTrue()
    whenIp4NotInNet4ThenFalse()
    whenIp4ItselfThenTrue()
    whenIp6InNet6ThenTrue()
    whenIp6NotInNet6ThenFalse()
    whenIp6ItselfThenTrue()
    whenNumericIp6InNet4ThenFalse()
    whenNumericIp4InNet6ThenFalse()
    whenIpInOneNetworkThenTrue()
    whenNoIpThenFalse()
    whenNoNetworkThenFalse()
