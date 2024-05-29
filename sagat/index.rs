# Faddy's Sagat/Kick Synthesizer

?# mkdir -p .FaddysSagat

?# cat - > .FaddysSagat/index.orc

+==

sr = 48000
ksmps = 32
nchnls = 2
0dbfs = 1

instr 13, sagat

iNumberOfObjects init 4
iDamp init 5
iShakingEnergy init 6
iPitch init 7

aNote tambourine 1, p3, \
p ( iNumberOfObjects ), \ ; defaults to 32
p ( iDamp ), \ ; ranges between 0 and .75
p ( iShakingEnergy ), \ ; defaults to 0, ranges between 0 and 1
2300 + cent ( 9 + p ( iPitch ) ), \ ; defaults to 2300
5600 + cent ( 10 + p ( iPitch ) ), \ ; defaults to 5600
8100 + cent ( 11 + p ( iPitch ) ) ; defaults to 8100

aNote clip aNote, 1, 1

outs aNote, aNote

endin

-==

?# cat - > .FaddysSagat/index.js

+==

import Scenarist from '@faddys/scenarist';

await Scenarist ( new class {

$_producer ( $ ) {

const sagat = this;
let version = 0;

for ( let pitch = 0; pitch < 12; pitch++ ) {

let index = ( ++version / 1000 ) .toString () .slice ( 2 );

index += '0' .repeat ( 3 - index .length );

let score = `.FaddysSagat/${ index }.sco`;

console .log ( [

`echo "i 13 0 0.03125 32 0 0 ${ pitch * 100/12 }" > ${ score }`,
`csound -o sagat-${ index }.wav .FaddysSagat/index.orc ${ score }`,
`aplay sagat-${ index }.wav`

] .join ( ' ; ' ) );

}

}

} );

-==

?# node .FaddysSagat/ > .FaddysSagat/index.sh

?# bash .FaddysSagat/index.sh
