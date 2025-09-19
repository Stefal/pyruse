# Exemples de logs str2str

## str2str_tcp

### str2str_tcp qui perd la connexion avec le récepteur
```
Sep 18 16:02:12 basegnss run_cast.sh[351714]: 2025/09/18 16:02:30 [CC---]  384257636 B   39534 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:03:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:03:12 basegnss run_cast.sh[351714]: 2025/09/18 16:03:30 [CC---]  384579996 B   39369 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:04:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:04:12 basegnss run_cast.sh[351714]: 2025/09/18 16:04:30 [CW---]  384600732 B       0 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:05:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:05:12 basegnss run_cast.sh[351714]: 2025/09/18 16:05:30 [CW---]  384600732 B       0 bps (0) /dev/ttyGNSS (1) waiting...
```

### str2str_tcp avec un F9P qui n'a pas de câble antenne
```
Sep 18 16:20:49 basegnss str2str_tcp[444711]: 2025/09/18 16:21:08 [CC---]     501772 B   14438 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:20:54 basegnss str2str_tcp[444711]: 2025/09/18 16:21:13 [CC---]     510856 B   14501 bps (0) /dev/ttyGNSS (1) 127.0.0.1
```

### Démarrage de str2str_tcp à "froid" sans limites sur les logs
```
Sep 18 16:36:27 basegnss str2str_tcp[460198]: stream server start
Sep 18 16:36:28 basegnss str2str_tcp[460198]: 2025/09/18 16:36:46 [CW---]          0 B       0 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:36:33 basegnss str2str_tcp[460198]: 2025/09/18 16:36:51 [CW---]      10792 B   14183 bps (0) /dev/ttyGNSS (1) waiting..
```

### Démarrage de str2str_tcp à "froid" avec limite sur les logs (LogRateLimitIntervalSec=10 + LogRateLimitBurst=1)
```
Sep 18 16:42:53 basegnss str2str_tcp[465163]: stream server start
Sep 18 16:43:04 basegnss systemd-journald[384]: [🡕] Suppressed 2 messages from str2str_tcp.service
Sep 18 16:43:04 basegnss str2str_tcp[465163]: 2025/09/18 16:43:22 [CW---]      18856 B   13853 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:43:14 basegnss systemd-journald[384]: [🡕] Suppressed 1 messages from str2str_tcp.service
Sep 18 16:43:14 basegnss str2str_tcp[465163]: 2025/09/18 16:43:32 [CW---]      36744 B   14271 bps (0) /dev/ttyGNSS (1) waiting...
```
## str2str_ntrip
### Démarrage de str2str_ntrip_A à "froid" avec une adresse de caster erronée
```
Sep 19 14:46:58 basegnss str2str_ntrip_A[691471]: 2025/09/19 14:47:17 [C----]          0 B       0 bps (0) localhost (1) address error (caster.centipede.frr)
Sep 19 14:47:03 basegnss str2str_ntrip_A[691471]: 2025/09/19 14:47:22 [C----]       9140 B   14644 bps (0) localhost (1) address error (caster.centipede.frr)
Sep 19 14:47:08 basegnss str2str_ntrip_A[691471]: 2025/09/19 14:47:27 [C----]      18216 B   14381 bps (0) localhost (1) address error (caster.centipede.frr)
```

### Perte de connexion au caster
```
Sep 19 15:06:47 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:06 [CC---]     788032 B   47386 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:06:52 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:11 [CC---]     816264 B   44592 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:06:57 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:16 [CC---]     844720 B   43358 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:07:02 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:21 [CW---]     873432 B   42942 bps (0) localhost (1) timeout
Sep 19 15:07:07 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:26 [CW---]     900424 B   45700 bps (0) localhost (1) timeout
Sep 19 15:07:12 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:31 [CW---]     930308 B   46390 bps (0) localhost (1) connecting...
Sep 19 15:07:17 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:07:36 [CW---]     957616 B   43697 bps (0) localhost (1) connecting...
```

### Reprise de connexion : 
```
Sep 19 15:08:52 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:11 [CW---]    1495828 B   42501 bps (0) localhost (1) connecting...
Sep 19 15:08:57 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:16 [CW---]    1524484 B   50004 bps (0) localhost (1) connecting...
Sep 19 15:09:02 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:21 [CW---]    1553188 B   45523 bps (0) localhost (1) connecting...
Sep 19 15:09:07 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:26 [CW---]    1580012 B   45262 bps (0) localhost (1) ERROR - Mount Point Taken or Inv
Sep 19 15:09:12 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:31 [CW---]    1609596 B   45409 bps (0) localhost (1) ERROR - Mount Point Taken or Inv
Sep 19 15:09:17 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:36 [CW---]    1636696 B   42305 bps (0) localhost (1) connecting...
Sep 19 15:09:22 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:41 [CC---]    1666652 B   42942 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:09:27 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:46 [CC---]    1694452 B   45910 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:09:32 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:51 [CC---]    1724372 B   50132 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
Sep 19 15:09:37 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:56 [CC---]    1751244 B   45304 bps (0) localhost (1) castertest.rtkbase.eu/MEUG
```

### Mount Point Taken or Inv
```
Sep 19 15:09:07 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:26 [CW---]    1580012 B   45262 bps (0) localhost (1) ERROR - Mount Point Taken or Inv
Sep 19 15:09:12 basegnss str2str_ntrip_A[698976]: 2025/09/19 15:09:31 [CW---]    1609596 B   45409 bps (0) localhost (1) ERROR - Mount Point Taken or Inv
```