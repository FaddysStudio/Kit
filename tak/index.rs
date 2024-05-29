# Faddy's Tak/Kick Synthesizer

?# mkdir -p .FaddysTak

?# cat - > .FaddysTak/index.orc

+==

sr = 48000
ksmps = 32
nchnls = 2
0dbfs = 1

giStrikeFT ftgen 0, 0, 256, 1, ".FaddysTak/marmstk1.wav", 0, 0, 0
giVibratoFT ftgen 0, 0, 128, 10, 1

instr 13, tak

aNote = 0

iPitch init frac ( p1 )

iAttack init 1/256
iDecay init 1/256 
iRelease init 1/32

aAmplitude linseg 0, iAttack, 1, iDecay, .25, iRelease, 0

aMainFrequency linseg cpsoct ( 11 + iPitch ), iAttack/2, cpsoct ( 7 + iPitch )
aHighFrequency linseg cpsoct ( 13 ), iAttack/2, cpsoct ( 9 )

aMain poscil aAmplitude, aMainFrequency
aHigh poscil aAmplitude, aHighFrequency

aNote += aMain + aHigh/4

aGogobell gogobel 1, cpsoct ( 7 ), .5, .5, giStrikeFT, 6.0, 0.3, giVibratoFT

aNote += aGogobell * aAmplitude

aSnatchAmplitude linseg 0, iAttack/2, 1, iDecay/4, 0
aSnatchFrequency linseg cpsoct ( 12 + iPitch ), iAttack/2, cpsoct ( 10 + iPitch )

aSnatch noise aSnatchAmplitude, 0
aSnatch butterlp aSnatch, aSnatchFrequency

aNote += aSnatch*3

aNote clip aNote, 1, 1

outs aNote, aNote

endin

-==

?# cat - > .FaddysTak/index.js

+==

import Scenarist from '@faddys/scenarist';

await Scenarist ( new class {

$_producer ( $ ) {

const tak = this;

tak .degrees = parseInt ( process .argv .slice ( 2 ) .pop () ) || 12;

tak .prepare ();

}

prepare () {

const tak = this;

for ( let step = 0; step < tak .degrees; step++ ) {

let index = ( step / 100 ) .toString () .slice ( 2 );

index += '0' .repeat ( 2 - index .length );

let score = `.FaddysTak/${ index }.sco`;

console .log ( [

`echo "i [ 13 + ${ step / tak .degrees } ] 0 1" > ${ score }`,
`csound -o tak-${ index }.wav .FaddysTak/index.orc ${ score }`,
`aplay tak-${ index }.wav`

] .join ( ' ; ' ) );

}

}

} );

-==

?# node .FaddysTak/ > .FaddysTak/index.sh

?# bash .FaddysTak/index.sh
