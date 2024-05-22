#!/usr/bin/env roll

# Faddy's Kit Machine

## Syntax

```bash
roll machine.rs
```

?# cat - > .FaddysKit/machine.mjs

+==
import Scenarist from '@faddys/scenarist';
import { createInterface } from 'node:readline';
import { stdin as input } from 'node:process';

await Scenarist ( new class {

$_producer ( $ ) {

$ ( ... process .argv .slice ( 2 ) );

}

} );
-==

?# $ node .FaddysKit/machine.mjs > .FaddysKit/machine.sh
?# bash .FaddysKit/machine.sh | node .FaddysKit/machine.mjs audioFiles

sr = 44100
ksmps = 32
nchnls = 2
0dbfs = 1

instr 1, 2

SPath strget p4
aChannel [] diskin2 SPath

iAmplitude init p5

iChannel = p1
SChannel sprintf "channel/%d", iChannel

chnmix aChannel [ 0 ] * iAmplitude, SChannel

endin

instr 11, 12

iChannel init p4 - 10
SChannel sprintf "channel/%d", iChannel
aChannel chnget SChannel
aChannel clip aChannel, 1, 1

outch iChannel, aChannel
chnclear SChannel

p1 init int ( p1 ) + iChannel / 10

endin
