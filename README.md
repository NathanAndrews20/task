# Task

A Simple CLI Application to manage tasks, written in [Rust](https://www.rust-lang.org/).

## Usage

### Creating a Task Group

```console
task create-group <GROUP_NAME>
```

### Creating a Task

```console
task add <GROUP_NAME> [CONTENT]...
```

The first argument to the `add` command will be treated as the name of the task
group. If the task group does not exists, then a new task group will be created
and the task will be added to the new task group.

### Completing a Task

```console
task complete <GROUP_NAME> <NUMBER>
```

Mark the `<NUMBER>`'th task in the `<GROUP_NAME>` task group as completed.

### Listing All Tasks in a Task Group

```console
task list <GROUP_NAME>
```

This is the most basic form of the `task list` command. This command will print
all of the tasks in a task group to the terminal. Tasks that are completed will
have a strikethrough.

### Showing Task Group Progress

```console
task list -p <GROUP_NAME>
```

The `-p` flag will not list the tasks in the task group, but will instead print
a progress bar to the terminal which will show the percentage of tasks completed
in the specified task group

### Showing Progress of all Tasks

```console
task list -a
```

The `-a` flag will print a progress bar for all tasks groups. It should be noted
that the `-a` flag and the `<GROUP_NAME>` argument are mutually exclusive and
should not be passed together.

### Removing a Specific Task in a Task Group

```console
task remove <-n NUMBER> <GROUP_NAME>
```

The `-n` flag is a number specifying which task to remove from the given task
group.

### Removing all Completed Tasks from a Task Group

```console
task remove -c <GROUP_NAME>
```

The `-c` flag will remove all tasks marked as completed from the given task
group.

## Implementation

### Storing Tasks

All tasks are stored in a `.tasks` directory located in the same directory as
the `task` executable. Tasks are stored in files in the `.tasks` directory where
each file represents one task group.

### Dependencies

This CLI app is developed in the [Rust programming language](https://www.rust-lang.org/)
and uses the following crates:

- [clap](https://crates.io/crates/clap): A command line argument parser
- [ansi_term](https://crates.io/crates/ansi_term): A library for ANSI terminal colors and styles
- [indicatif](https://crates.io/crates/indicatif): A progress bar and CLI reporting library
