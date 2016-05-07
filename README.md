summon-keepass
==============

Simple [summon](https://conjurinc.github.io/summon/) provider that allows usage of Keepass kdbx database file.

Usage
-----

Create `.summon-keepass.ini` file in your `$HOME` directory with the following content:

    [keepass_db]
    path=/path/to/your/keepass_database_file.kdbx
    pass=password to your keepass database

Place `summon-keepass` in `/usr/local/lib/summon`

Todo
----
- tests
- error handling for incorrect config/keepass file path
