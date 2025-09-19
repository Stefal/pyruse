# Exemples de logs str2str

## str2str_tcp qui perd la connexion avec le récepteur
Sep 18 16:02:12 basegnss run_cast.sh[351714]: 2025/09/18 16:02:30 [CC---]  384257636 B   39534 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:03:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:03:12 basegnss run_cast.sh[351714]: 2025/09/18 16:03:30 [CC---]  384579996 B   39369 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:04:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:04:12 basegnss run_cast.sh[351714]: 2025/09/18 16:04:30 [CW---]  384600732 B       0 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:05:12 basegnss systemd-journald[384]: [🡕] Suppressed 11 messages from str2str_tcp.service
Sep 18 16:05:12 basegnss run_cast.sh[351714]: 2025/09/18 16:05:30 [CW---]  384600732 B       0 bps (0) /dev/ttyGNSS (1) waiting...

## str2str_tcp avec un F9P qui n'a pas de câble antenne
Sep 18 16:20:49 basegnss str2str_tcp[444711]: 2025/09/18 16:21:08 [CC---]     501772 B   14438 bps (0) /dev/ttyGNSS (1) 127.0.0.1
Sep 18 16:20:54 basegnss str2str_tcp[444711]: 2025/09/18 16:21:13 [CC---]     510856 B   14501 bps (0) /dev/ttyGNSS (1) 127.0.0.1

## Démarrage de str2str_tcp à "froid" sans limites sur les logs
Sep 18 16:36:27 basegnss str2str_tcp[460198]: stream server start
Sep 18 16:36:28 basegnss str2str_tcp[460198]: 2025/09/18 16:36:46 [CW---]          0 B       0 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:36:33 basegnss str2str_tcp[460198]: 2025/09/18 16:36:51 [CW---]      10792 B   14183 bps (0) /dev/ttyGNSS (1) waiting..

## Démarrage de str2str_tcp à "froid" avec limite sur les logs (LogRateLimitIntervalSec=10 + LogRateLimitBurst=1)
Sep 18 16:42:53 basegnss str2str_tcp[465163]: stream server start
Sep 18 16:43:04 basegnss systemd-journald[384]: [🡕] Suppressed 2 messages from str2str_tcp.service
Sep 18 16:43:04 basegnss str2str_tcp[465163]: 2025/09/18 16:43:22 [CW---]      18856 B   13853 bps (0) /dev/ttyGNSS (1) waiting...
Sep 18 16:43:14 basegnss systemd-journald[384]: [🡕] Suppressed 1 messages from str2str_tcp.service
Sep 18 16:43:14 basegnss str2str_tcp[465163]: 2025/09/18 16:43:32 [CW---]      36744 B   14271 bps (0) /dev/ttyGNSS (1) waiting...