# Backlog

* Switch the `dailyReport` from a static layout to a light-weight template system.
* Change `e.get("D", Details.ALL.name)` to `e["D"]` in `action_dailyReport.py` at release next+1, when backward compatibility with the running Pyruse will not be an issue any more.
* Maybe persist counters; they are currently lost on restart.
* Maybe switch from storing the daily journal in a file, to storing it in a database.
* Eventually make the code more elegant, as I learn more about Python…
