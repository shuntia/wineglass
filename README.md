# wineglass
Wineglass, a dynamic object-oriented language, interpreted by rust.

## Bottle
Each bottle includes a baked in parser that executes code sequentially, wherever you call it.
The bottle does preprocessing checks with "Winecellar" to check if the attaching site is valid.

### Exceptions

Bottles will "Spill" whenever there are exceptions unhandled within the bottle.

### Importing

The Bottle is also responsible for importing, so whenever you import a new module dynamically, it will take time.
The Bottles for importing will turn into a packaged tree for the other programs to execute, which is generally global.

There are three types of dependencies.
#### Require
This means a module WILL NOT work without the dependency running, and therefore the bottle will spill if the module is unable to be loaded.

#### Lock
This means that the thread of the bottle will lock until the target bottle is activated or is running.

#### Import
This means that the module is not required but will be loaded upon calls to that module.

