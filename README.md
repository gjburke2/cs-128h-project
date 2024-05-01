# cs-128h-project

## Group Info

Name: G&G

Memeber Names: Griffin Burke and Gabriel Schwarz and Kaden Paris

NetIDs: gjburke2 and gabe2 and kadenp2

## Project Introduction

We are looking to build a command-line-based task manager application. This will be a good way to work with classes and apply multi-threading. We've chosen to work on it because it is a useful application for keeping track of events and time.

## Technical Overview

In the application, the user will be able to create, store, and load different lists of tasks. In the creation of a list of tasks, users will be able to name the list. For each task, the user will be able to set a name, length, and priority value. When a list of tasks is loaded, the user will be able to start the tasks and pause them if needed. The task will last as long as they specified for the length. Multiple tasks can go on at once, as they will be multi-threaded. When a task finished it will mark itself as complete.

Breakdown of what large chunks need to be done:
- TaskList class          |  Checkpoint 1
- Task class (w/o timing) |  Checkpoint 1
- Multithreading logic    |  Checkpoint 2
- User interaction        |  Checkpoint 2

## Possible Challenges

- Multithreading with keeping track of timing left and pausing may be tough. Have to keep ownership in check and be able to update the time left
- Making the interaction with the command-line streamlined and with not too many options

