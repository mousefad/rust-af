af (in Rust)
============

Usage
-----

::
  af [-F delim_regex] [-d output_delimiter] field_id [field_id ...] [path [path ...]]

Reads from ``path`` if specifed or standard input if not. Reads a line at a time, splits with ``delim_regex`` and
outputs numbers fields where ``field_id`` either:

* A positive integer: will select the *nth* field left to right
* A negative integer: will select the *nth* field right to left
* ``NF`` means the right-most field (alias for -1)

Remaining positional parameters are assumed to be ``path`` arguments. If you want to read a file in the current directory
that looks like an integer or "NF" then just prefix it with ``./``.

The net result of all this is that this command::

  $ echo " first  second third     fourth" | af 3 4

Approximates the behaviour of::

  $ echo " first  second third     fourth" | awk '{ print $3 " " $4 }'

"Approximates" because Rust's regex dialect is different from that used by ``awk``.


Field IDs can be 

Options
-------

==== ==============================================================
Flag Behaviour
==== ==============================================================
-F   Sets the input field separator regex (default is ``\s+``)
-d   Sets output field separator string (default is a single space)
==== ==============================================================


Background
----------

Awk is a pretty neat and flexible little language. However, what I most often use it for is to extract fields that
are separated by whitespace::

  $ echo "first second third fourth" | awk '{ print $3 " " $4 }'
  third fourth

One might argue that this is what ``cut``` is for, which is all well and good when fields are delimited with a
single space or single tab::

  $ echo "first second third fourth" | cut -d" " -f 3,4
  third fourth

However, quite often I don't have such well-behaved inputs, and ``cut`` doesn't do quite what I want, whereas the
``awk`` approach still works "properly"::

  $ echo "first  second third   fourth" | cut -d" " -f 3,4
  second third

  $ echo "first  second third   fourth" | awk '{ print $3 " " $4 }'
  third fourth

We can fix the `cut` approach by using ``sed`` to normalize the separators, but to handle arbitrary amounts of
mixed tabs and spaces and/or leading whitespace, it gets a bit much::

  $ echo -e " first \tsecond\tthird  \t fourth" | sed -e 's/^[\t ]*//' -e 's/[\t ][\t ]*/ /g' | cut -d" " -f 3,4
  third fourth

There are doubtless an endless number of ways to accomplish the same thing, but what I habitually reach for is
``awk`` because it's reliable and easy (if a little unergonomic). At some point along the way I wrote a shell script
``af`` which took just the field numbers (or "NF" for the last field) and generated the ``awk`` script and then ran
it. Then I re-wrote it in ``go`` and now I'm doing it in Rust. Why? I don't have to explain myself to the likes of you!






