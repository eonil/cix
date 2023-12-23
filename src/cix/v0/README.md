

CIX V0
======

- `arb`: Scheduler, interactor server.
- `xct`: Build executor client.
- `dpt`: Build artifact storage server.

Design Choices
--------------
- `arb` just arbitrate parties.
- `xct` runs on client machine. (macOS)
- `xct` connects to `arb` and take one pending job.
- `xct` runs alone while executing job.
  - Therefore, job execution continues even `arb` dies.
  - Therefore, `arb` can die, reboot at any time.


