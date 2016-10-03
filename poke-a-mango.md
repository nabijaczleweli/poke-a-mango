poke-a-mango(1) -- What all the kool kidz are playing these days
================================================================

## SYNOPSIS

`poke-a-mango` [OPTIONS]

## DESCRIPTION

Played with your friends or alone, this game provides great entertainment to
everyone all over the globe, requiring great eye-hand coordination and keeping
the atmosphere tense.

Exit values and possible errors:

    1 - Failed to parse a file
    2 - An I/O error occured
    3 - UI failed to cooperate

## OPTIONS

  -c --config-dir &lt;<config_dir>&gt;

    Directory with the configuration.

    The configuration directory contains all of poke-a-mango's data.

    Default: $HOME/.poke-a-mango

  -d --desktop-size &lt;<size>&gt;

    The target desktop's resolution.

    By default this is autodetected to match the primary monitor's resolution,
    but can be overriden to scale the game window better.

    Format: NxM

## EXAMPLES

  `poke-a-mango` `-d` *1280x720*

    Run the game as on an HD monitor.

  `poke-a-mango` `-c` *pkmngo*

    Save the game data in the pkmngo directory instead of the default one.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/poke-a-mango/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/poke-a-mango>&gt;
