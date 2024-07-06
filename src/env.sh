#!/bin/sh
# calcagebra shell setup
# affix colons on either side of $PATH to simplify matching
case \":${PATH}:\" in
    *:\"$HOME/.calcagebra/bin\":*);;
    *) export PATH="$HOME/.calcagebra/bin:$PATH";;
esac
