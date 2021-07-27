# CATnet
<p align="center">
  <a href="https://github.com/ZincSoft/CATNET" target="blank"><img src="assets/logo.png" alt="CATNET Logo" /></a>
  <img src="https://img.shields.io/github/license/ZincSoft/CATNET?:ZincSoft:/CATNET?label=License&style=flat" alt="License">
  <img src="https://img.shields.io/github/stars/ZincSoft/CATNET?:ZincSoft:/CATNET?label=Stars&tyle=flat" alt="Stars">
  <img src="https://img.shields.io/github/watchers/ZincSoft/CATNET?style=social&label=Watchers&style=flat" alt="Watchers">
  <img src="https://img.shields.io/github/forks/ZincSoft/CATNET?style=social&label=Forks&style=flat" alt = "Forks">
  <img src="https://img.shields.io/codefactor/grade/github/ZincSoft/CATNET?label=Code%20Factor%20Grade" alt="Code Factor Grade">
  <img src="https://img.shields.io/badge/Linux-Supported-green" alt="Linux Supported!">
  <img src="https://img.shields.io/badge/MacOS-Supported-green" alt="MacOS Supported!">
  <img src="https://img.shields.io/badge/Windows-Nope.%20Support%20Planned-red" alt="Fuck Windows!">
  <img src="https://img.shields.io/github/last-commit/ZincSoft/Catnet.svg" alt="Last Commit">
  <img src="https://img.shields.io/aur/last-modified/Catnet.svg" alt="AUR Last Modified">
</p>
CATnet is a more desentralized web for a more modern age.

## User use
### Base Installation
![build and install instructions](assets/build.svg)
```bash
cargo install --git https://github.com/ZincSoft/CATNET.git
```

### Use
#### Participant
Look at the help menu, then run with the flags/arguments you want!
##### Note: You may want to install a graphical client! You can find these in src/clients, or as a release.
![Participant](assets/participant.svg)
```bash
catnetd participant -l3
```

#### Registrar
Look at the help menu, then run with the flags/arguments you want!
![Registrar](assets/registrar.svg)
```bash
catnetd registrar -l3
```


## Software Development
### Usefull flags
During development of CATnet, you may wish these flags:
* `-l0`,           enables all levels of logging. Please note that *trace* and *info* logging are disabled in release builds.

### Git Workflow
Pretend the other people working on this code base are insane, have a shotgun, and know where you live. As such, please follow `WORKFLOW.MD`.

### White paper
[Here](https://docs.google.com/document/d/1t3FXJTDr-h4J9iPvzBLDdCKGJAukKruhrJjNaMWRgq0/edit?ts=5fc41d5f#heading=h.3bqhl2hpdgyy) is a link to our whitepaper document.

### Contributing
[![GitHub last commit](https://img.shields.io/github/last-commit/zincsoft/catnet.svg?style=flat)]()
[![GitHub commit activity the past week, 4 weeks](https://img.shields.io/github/commit-activity/y/zincsoft/catnet.svg?style=flat)]()
[![GitHub commits since](https://img.shields.io/github/commits-since/zincsoft/catnet/v1.2.0.svg)]()
[![Issues](https://img.shields.io/github/issues-raw/tterb/PlayMusic.svg?maxAge=25000)](https://github.com/zincsoft/catnet/issues)  
[![Pull Requests](https://img.shields.io/github/issues-pr/zincsoft/catnet.svg?style=flat)]()
[![PR's Welcome :D](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat)](http://makeapullrequest.com)  
Thank you for even considering contributing to this project. At this time, we will *NOT* be merging pull requests that are linked to issues that themselves are linked to a certain project. We are, however, eternally greatfull for bug fixes.

