# Rorg-Agenda

**WARNINGS**
- Rorg-agenda is in an early stage in development, everything can change
in the future from the name of the variable to the way you interact with it,
passing by the way it formats these files.
- I not a native English speaker so there may be spelling and grammar
issues in the software. Don't hesitate to tell me if you see one..


## What is this?

Rorg-Agenda is a standalone time management system that uses .org files to store data.
Now, Rorg-Agenda can generate the files and read them.
## Usage

```
$ rorg [action-arg] [specifier]
```

|action arguments |specifier                      |output            |
|:---------       |:--------                      |:-----            |
| --init          |                               |initialize the directory|
| --read          |(file to read)                 |read a file       |
| --add           |(Info to generate the entry)   |add an entry      |
| --remove        |(Info to identify the entry)   |remove an entry   |
