# worktracker

A command line tool that calculates total time based on timestamps written in a simple human-readable format. It can be used to track time spent on work or any other kinds of tasks.

## How to use

The tool works in a similar way to `wc` and accepts input from stdin.

`~/.log.txt`

```
1 10
2 45

11:23
1:30

13h 10m
13h 50m
```

```console
$ worktracker < ~/.log.txt

 1h 10m -  2h 45m => 1h 35m
11h 23m -  1h 30m => 2h 7m
13h 10m - 13h 50m => 40m

Total time: 4h 22m
```

You can also run `worktracker` and type the timestamps manually or paste them into the terminal and press `ctrl+z` to simulate EOF.

## Syntax

The tool works with both **12h** and **24h** time formats.

Supported timestamp formats `11 20`, `11:20`, `11h 20m`.

**TODO:** add support for timestamps without minutes (`11h`)

The timestamps should be written in pairs. The first one is **start**, the second one is **end**. Each pair should be separated by a newline.

Timestamps with incorrect syntax are ignored.

## License

MIT
