# Changelog

This file is not intended as a dupplicate of Git logs.
Its purpose is to warn of important changes between version, that users should be aware of.

## v2.0

After this version is installed, the following command should be run on the `action_nftBan.py.json` file:

```bash
$ sudo sed -i s/nftSet/nfSet/g action_nftBan.py.json
```
