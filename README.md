rust-GSL
========

A `Rust` binding for [GSL library](http://www.gnu.org/software/gsl/).

##Installation

This binding requires the `GSL` library to be installed.

To build it, please use :

```Shell
> make
```

This command build __rgsl__, the examples and the documentation.

You can build them separatly too.

```Shell
> make rgsl
> make examples
> make doc
```

##Documentation

You can access the __rgsl__ documentation locally, just build it :

```Shell
> make doc
```

Then open this file with an internet browser :
file:///{rgsl_location}/doc/rgsl/index.html

## License
`rust-GSL` is a wrapper for `GSL`, therefore inherits the [GPL licence](http://www.gnu.org/copyleft/gpl.html).