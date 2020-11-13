# swi_prolog

This crate try build bindings to SWI-Prolog.h interface in a rusty way.
SWI-Prolog.h is often provided when `swipl` binary is installed.
This is absolutely an amateur project and have no relation with SWI Prolog developers.
This crate is named in this way simply because it links using SWI-Prolog.h interface.
Since this project is amateur, expect bugs and crash, so do not use on critical systems.


Supported features:
- [X] Rusty safe interface
- [X] Multithread
- [X] Async interface

Known bugs:
- Running test crate, sometimes a sigabort/sigsev is throwed. This means that there is some multithreading error.


