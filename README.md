summon-keepass
==============

Simple [summon](https://conjurinc.github.io/summon/) provider that allows usage of Keepass kdbx database file.

Installation
-----

Create `.summon-keepass.ini` file in your `$HOME` directory with the following content:

    [keepass_db]
    path=/path/to/your/keepass_database_file.kdbx
    pass=password to your keepass database

Place `summon-keepass` in `/usr/local/lib/summon`

Usage
-----

Let's say you have the following entries in your `secrets.yml` file:

    AWS_ACCESS_KEY_ID: !var aws/iam/user/robot/access_key_id
    AWS_SECRET_ACCESS_KEY: !var aws/iam/user/robot/secret_access_key

`summon-keepass` will split each secret with `/` and then return password from database entry when username match last part of the secret and is placed in correct group determined by previous parts of the secret.

In that case Keepass database should look like this:
![Keepass example](http://i.imgur.com/WwkYYSG.png)

Todo
----
- tests
- error handling for incorrect config/keepass file path
