# Code-header-gen

> A Generator for header comments at the beginning of a source code file

## Configuration

### Global

Global configuration should be in `XDG_CONFIG_HOME/.hgen`:

```
XDG_CONFIG_HOME/hgen
|-- settings.json       (hgen Settings)
|-- languages.json      (Language configuration)
`-- templates           (Templates)
    |-- master.hbs      (Master template, used if language specific template is not defined)
    `-- vhdl.hbs        (Language specific template for VHDL)
```

### Local

Just copy [`hgen`](./.hgen) to the project directory

### Configuration files

#### [`settings.json`](./.hgen/settings.json)

Date format should be written according to https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html

#### [`languages.json`](./.hgen/languages.json)

#### [`master.hbs`](./.hgen/templates/master.hbs)

##### Predefined template keywords

| Keyword    | Description   |
|:----------:|:--------------|
| `{{lc}}`   | line comment  |
| `{{date}}` | date          |
| `{{fn}}`   | file name     |
| `{{lang}}` | file language |

## Usage

TODO