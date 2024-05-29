# Faddy's Dom/Kick Synthesizer

?# mkdir -p .FaddysDom

?# cat - > .FaddysDom/index.orc

+==

sr = 48000
ksmps = 32
nchnls = 2
0dbfs = 1

giStrikeFT ftgen 0, 0, 256, 1, ".FaddysDom/marmstk1.wav", 0, 0, 0
giVibratoFT ftgen 0, 0, 128, 10, 1

instr 13, dom

aNote = 0

iPitch init frac ( p1 )

iAttack init 1/64
iDecay init 1/16 
iRelease init 1/2

aMainSubAmplitude linseg 0, iAttack, 1, iDecay, .25, iRelease, 0
aMainSubFrequency linseg cpsoct ( 8 + iPitch ), iAttack, cpsoct ( 5 + iPitch )

aMainSub poscil aMainSubAmplitude, aMainSubFrequency

aNote += aMainSub

aHighSubAmplitude linseg 0, iAttack, 1, iDecay, .25, iRelease/8, 0
aHighSubFrequency linseg cpsoct ( 13 + iPitch ), iAttack/2, cpsoct ( 6 + iPitch )

aHighSub poscil aHighSubAmplitude, aHighSubFrequency

aNote += aHighSub / 4

/*

aTambourine tambourine 1, 1/128, 16, 1/2, 1

aTambourine = aTambourine * .25

aNote += aTambourine

*/

aGogobell gogobel 1, cpsoct ( 6 + iPitch ), .5, .5, giStrikeFT, 6.0, 0.3, giVibratoFT

aNote += aGogobell / 4

aSnatchAmplitude linseg 0, iAttack/2, 1, iDecay/4, 0
aSnatchFrequency linseg cpsoct ( 10 + iPitch ), iAttack/2, cpsoct ( 9 + iPitch )

aSnatch noise aSnatchAmplitude, 0
aSnatch butterlp aSnatch, aSnatchFrequency

aNote += aSnatch

aNote clip aNote, 1, 1

outs aNote, aNote

endin

-==

?# cat - > .FaddysDom/index.js

+==

import Scenarist from '@faddys/scenarist';

await Scenarist ( new class {

$_producer ( $ ) {

const dom = this;

dom .degrees = parseInt ( process .argv .slice ( 2 ) .pop () ) || 12;

dom .prepare ();

}

prepare () {

const dom = this;

for ( let step = 0; step < dom .degrees; step++ ) {

let index = ( step / 100 ) .toString () .slice ( 2 );

index += '0' .repeat ( 2 - index .length );

let score = `.FaddysDom/${ index }.sco`;

console .log ( [

`echo "i [ 13 + ${ step / dom .degrees } ] 0 1" > ${ score }`,
`csound -o dom-${ index }.wav .FaddysDom/index.orc ${ score }`,
`aplay dom-${ index }.wav`

] .join ( ' ; ' ) );

}

}

} );

-==

?# node .FaddysDom/ > .FaddysDom/index.sh

?# bash .FaddysDom/index.sh
