# Rorg-Agenda

**WARNINGS**

- I'm not activelly working on it. If you're looking for a good cli/tui organizer
  take a look at [Taskwarior](https://taskwarrior.org/) and [taskwarior-tui](https://github.com/kdheepak/taskwarrior-tui) for a Tui.
  

- Rorg-agenda is in an early stage in development, everything can change
in the future from the name of the variable to the way you interact with it,
passing by the way it formats these files.
- I not a native English speaker so there may be spelling and grammar
issues in this repo. Don't hesitate to tell me if you see one..

## What this will be?
Rorg-Agenda is a standalone time management system that uses .org files to store data.
It will be able to help you if you don't want to open emacs each time you want
to add  or see something in your To-do-list, if you want to automate the addition
of some event (using Caldav for example) or if you want to have notification about
what you have to do.

At this point, Rorg-agenda is unable to do anything except add task to a to-do list
## Usage

```
$ rorg [action-arg] [specifier]
```

|action arguments |specifier                      |output            |
|:---------       |:--------                      |:-----            |
| --init          |                               |initialize the directory|
| --read          |(file to read)                 |read a file       |
| --add           |(Info to generate the entry)   |add an entry      |
| --tui           |                               |use the terminal interface|
