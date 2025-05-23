# Alfred Form Generator

The application `afg` is used to generate `Swift` code 
from a specification. The generated `Swift` 
code is a standalone application that presents the user with a form 
to provide answers that can then be parsed by `Alfred` and take
appropriate actions.

The application prints the entered values to `STDOUT` as tab separated
key value pairs.

## Usage

```bash
Alfred Form Generator

Usage: afg [OPTIONS] --title <title> --field <field> <output>

Arguments:
  <output>  The name of the swift output file

Options:
  -v                                    turns on verbose mode
      --separator <field_separator>     Specify the field separator [default: @]
      --templates <template_directory>  Specify the directory of templates [default: Template]
  -t, --title <title>                   Specify the field separator
  -f, --field <field>                   Describes details of a field
  -h, --help                            Print help
  -V, --version                         Print version

This application is used to generate a swift application that can be run in Alfred as a form input
```

## Specification

The specification should at least specify the `title`, one form `field` and
the output file.

The `title` is a free format title that should specify the goal of the form.

### Field

The form `field` option has 4 or 5 fields depending on the type
of field.

|Element| Description                                                                          |
|-|--------------------------------------------------------------------------------------|
|type| The type of form field. Possible values are "string", "bool" and "picker".           |
|name| The name of the form field. The name is used to return the value.                    |
|default| The default value. This value can be empty, but should be specified without a value. |
|decoration| The annotation (description) of the field in the form.                               |
|choices| The possible values of the picker. This is only valid for type=picker.               |

An example of the field specification is as follows:

```bash
--field type=String@name=title@default=example@decoration="The Title"
```

The keys are case insensitive. The default separator is `@` and can be
overruled with the option `--separator`.

## An example

A full example could be as follows:

```bash
afg \
	--title "The example title" \
	--field type=String@name=title@default=example@decoration="The Title" \
	--field type=String@name=subtitle@default=@decoration="The Subtitle" \
	--field type=Bool@name=enabled@default=true@decoration="Enable secret" \
	--field type=Picker@name=choice@default=0@decoration="Make your choice"@choices="Choice A,Choice B,Choice C" \
	--field type=Picker@name=choice2@default=2@decoration="Make another choice"@choices="Choice A,Choice B,Choice C" \
	output.swift
```

The then generated output file can then be run using the command:

```bash
swift output.swift
```

An example output of the above command is:

```bash
title=This is the entered title subtitle=This is the subtitle enabled=true choice=B choice2=C
```

Note that the output is tab separated.

when the dialog is cancelled, the output is the word "canceled".

## Templates

The application uses two templates. 
## Credits

The original idea to use `Swift` code to generate a UI is coming
from Patrick Sy [GUI Input Experiment](https://github.com/zeitlings/alfred-workflows/releases/tag/v1.0.0-uiex)

The templates are based on the original work of Patrick.

The main difference is that the code from Patrick was static 
and did not have default keys.
