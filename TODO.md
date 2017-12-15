# TODO

* Insert the GPL stuff in the source files.
* Create a filter that rejects all messages that match a series of regular expressions.
* Handle OTHER messages in a file in order to avoid filling the RAM waiting for the report to be sent.
* Write the systemd service that starts pyruse on boot.
* Write the systemd service+timer that restores bans after a reboot.
* Maybe switch from Step.run() recursion to Step.run()-in-a-loop to avoid too-deep call stacks.
* Eventually make the code more elegant, as I learn more about Python…
