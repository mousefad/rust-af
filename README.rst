af: "awk field" filter (in Rust)
================================

Usage::

  af [OPTIONS] N1 [N2 ...] [file [file ...]]


Reads lines of input and splits them with the *input delimiter regex* (default "\s+"). Then for each *field 
pospition* ``N1. N2 ...``, copies the value of split field to standard output, separated by the *output delimiter 
string* (default=" "). 

Field positions ``N1, N2 ...`` make each be one of the following:

* A positive integer: will select the *nth* field left to right starting at ``1``
* A negative integer: will select the *nth* field right to left starting at ``-1``
* ``NF`` means the right-most field (alias for ``-1``)

Any positional parameters that do not match one of the above will be added to a list of filenames to process. If no 
filenames are provided as positional parameters, data will be read from standard input. 


Options
-------



===========================  ============================================================================
Flag                         Behaviour
===========================  ============================================================================
-F, --input-delimiter RX     Sets the *input delimiter regex* to ``RX``. The default is ``\s+``
-d, --output-delimiter STR   Sets the *output delimiter string* to ``STR`` (default is a single space).
-b, --print-blanks           Do not skip printing of empty output lines (default is to skip).
===========================  ============================================================================


Examples
--------

Show filesystem fullness by mount point::

  $ df -hP | af 5 6


Get a list of UID and corresponding usernames::

  $ af -F : 3 1 /etc/passwd


It gets a little trickier if we want to use negative field positions because the command line option parser will
attempt to interpret options starting with ``-`` as option flags.  This can be suppressed by using ``--`` (after
ehich the parser will treat all remaining arguments as positional parameters)::

  $ df -hP | af -- -2 -1


One other possible complication is when there are file names that look like field positions (numeric or ``NF``).
In these cases we can just prefix the filenames with ``./`` if they are in the current working directory::

  $ ls
  1  2  3

  # We want to print the 5th field from these three files, but "1 2 3" look like field numbers:
  $ af 5 1 2 3 

  # solution: prefix the file names with ./
  $ af 5 ./1 ./2 ./3


Background
----------

Awk is a pretty neat and flexible little language. However, what I most often use it for is to extract fields that
are separated by whitespace::

  $ echo "first second third fourth" | awk '{ print $3 " " $4 }'
  third fourth


One might argue that this is what ``cut``` is for, which is all well and good when fields are delimited with the
same between each field::

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

