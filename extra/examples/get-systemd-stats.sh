#!/bin/bash
# $1 (optional): systemd-nspawn machine name

{
	printf 'Units\tTotal\tP7\tP6\tP5\tP4\tP3\tP2\tP1\tP0\n'
	sudo journalctl ${1:+-M "$1"} -o json-pretty --output-fields=_SYSTEMD_UNIT,PRIORITY \
	| tr -d $'"\t, ' \
	| awk -F: -vOFS=: '
		/^\{/ {
			u = ""
			p = -1
		}
		$1 == "PRIORITY" {
			p = $2
		}
		$1 == "_SYSTEMD_UNIT" {
			u = gensub(\
				"@.*(\\.[^.]*)$",\
				"@*\\1",\
				1,\
				gensub("-[^-]*[0-9][^-]*(\\.[^.]*)$", "-*\\1", 1, $2)\
			)
		}
		/^\}/ {
			if (p >= 0) print u, p
		}
	' \
	| sort \
	| awk -F: -vOFS=$'\t' '
		function out() {
			if (u != "")
				print u,\
					(p[8]+p[7]+p[6]+p[5]+p[4]+p[3]+p[2]+p[1]),\
					p[8], p[7], p[6], p[5], p[4], p[3], p[2], p[1]
			split("0:0:0:0:0:0:0:0", p)
			u = ""
		}
		$1 != u {
			out()
			u = $1
		}
		{
			p[1 + $2] += 1
		}
		END {
			out()
		}
	' \
	| sort -t$'\t' -k2,2rn
}
