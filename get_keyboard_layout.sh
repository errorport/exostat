#!/bin/bash
# Simply returns keyboard layout
# 2 letters, uppercase.
setxkbmap -query | grep layout | awk '{print toupper($2)}'
