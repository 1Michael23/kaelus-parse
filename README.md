# Kaelus-Parser

A CLI utility to read exports from Kaelus IVA cable sweep testers

### Usage

```$ ./kaelus_parse <flags> <path to report.xml>```

#### Flags

-v --verbose : Prints extra data about the report.
-s --sort    : Sorts the cables alphabetically by tag.

### Output

![simple example](./docs/simple_example.png?raw=true "Simple example")
![verbose example](./docs/verbose_example.png?raw=true "Simple example")

## Limitations

- Requires unziping file and targeting "Report.xml"
- Only Supports Distance to Fault, and Return Loss tests
- Probably Missing edgecases, submit an issue with the report if you find one
