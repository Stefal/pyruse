# Writing custom modules

Custom filters are Python files written in `/etc/pyruse/pyruse/filters/`.
Custom actions are Python files written in `/etc/pyruse/pyruse/actions/`.

Filters must define a class named `Filter` that extends Pyruse’s own `Filter` class from the `pyruse.base` namespace.
By convention, a filter module name starts with `filter_`. A filter module looks like this:

```python
from pyruse import base

class Filter(base.Filter):
  def __init__(self, args):
    super().__init__()
    # get mandatory arguments with args["param_name"]
    # get optional arguments with args.get("param_name", default_value)
    # store in self.whatever the data that is needed at each run of filter below

  def filter(self, entry):
    # return true for the "then" link, or false for the "else" link
    return some_check(entry["a_field"], entry["another_field"])
```

Actions must define a class named `Action` that extends Pyruse’s own `Action` class from the `pyruse.base` namespace.
By convention, an action module name starts with `action_`. An action module looks like this:

```python
from pyruse import base

class Action(base.Action):
  def __init__(self, args):
    super().__init__()
    # get mandatory arguments with args["param_name"]
    # get optional arguments with args.get("param_name", default_value)
    # store in self.whatever the data that is needed at each run of act below

  def act(self, entry):
    # do whatever this action is supposed to do
```

Some actions may need to restore a state at boot, or each time the main Pyruse program is restarted. The aim usually is to configure an external tool (firewall, etc.), based on files, or a database…
In such cases:

* The action’s constructor must be altered so that it does not fail if `args` is `None`:

```python
  def __init__(self, args):
    super().__init__()
    if args is None:
      return
```

* A new `boot` method must be defined; it will get called at boot and this is where the wanted state shall be restored:

```python
  def boot(self):
    # do whatever must be done
```

* Assuming the action is named `action_myModule`, the systemd unit `pyruse-boot@action_myModule.service` should be enabled. If this unit has dependencies, these must be declared before enabling the specific `pyruse-boot` service, by creating a drop-in with the dependencies, for example:

```
# /etc/systemd/system/pyruse-boot@action_myModule.service.d/action_myModule.conf
[Unit]
Requires=iptables.service
After=iptables.service
```
