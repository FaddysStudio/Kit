#!/usr/bin/env roll

# Faddy's Kit

A roll for playing and recording rhythms from provided sound kits and external audio input.

## Installation

Since this file is a roll, it requires Faddy's Roll to be installed.
In addition, Csound must be installed for this roll to work.

```sh
# Based on system permissions, It might be required to be run using sudo:
npm i -g @faddys/roll
```

## Usage

```bash
roll index.rs <step>...
```

## Implementation

Create a hidden directory to act as a scratch for the roll.

?# mkdir -p ._

### Csound Engine

#### Csound Orchestra Index

?# cat - > ._/index.orc

+==

sr = 48000
ksmps = 48
nchnls = 2
0dbfs = 1

#include "kit.orc"
#include "recorder.orc"
#include "rhythm.orc"
#include "beat.orc"

-==

#### Csound Kit Instrument

?# cat - > ._/kit.orc

+==

instr kit

SKit strget p4

SSize sprintf "%s/size", SKit
iSize chnget SSize

SIndex sprintf "%s/%d", SKit, iSize
SPath strget p5

chnset SPath, SIndex
chnset iSize + 1, SSize

endin

-==

### Csound Recorder Instrument

?# cat - > ._/recorder.orc

+==

instr recorder

aLeft, aRight ins

iTimeStamp date
SRecording sprintf "._/recording_%d.wav", iTimeStamp

fout SRecording, -1, aLeft, aRight

endin

-==

## Csound Rhythm Instrument

This instrument plays one of the beats passed as string p-fields once per cycle.
A cycle is the absolute value of p3.

```csound
i "rhythm" $delay $cycle "$beat1" ... "$beatN"
```

?# cat - > ._/rhythm.orc

+==

instr rhythm

iInstance chnget "instance"
iInstance += 1
chnset iInstance, "instance"

iInstance /= 1000
p1 init int ( p1 ) + iInstance

iBeat nstrnum "beat"

iCycle init abs ( p3 )
kRhythm metro 1 / iCycle

kSwing jspline 1, 0, 4
kSwing = abs ( kSwing ) * 127

if kRhythm == 1 && kSwing > p5 then

SKit strget p4

SNote sprintfk {{ i %f 0 1 "%s" }}, iBeat + iInstance, SKit

scoreline SNote, 1

endif

endin

-==

#### Csound Beat Player Instrument

?# cat - > ._/beat.orc

+==

instr beat

SKit strget p4
SSize sprintf "%s/size", SKit
iKit chnget SSize

iIndex random 0, iKit
SIndex sprintf "%s/%d", SKit, iIndex

SRhythm chnget SIndex

p3 filelen SRhythm

kPitch jspline 1, 0, 4

aRhythm [] diskin2 SRhythm, cent ( kPitch * 10 )

iChannels lenarray aRhythm

if iChannels == 1 then

aLeft = aRhythm [ 0 ]
aRight = aRhythm [ 0 ]

else

aLeft = aRhythm [ 0 ]
aRight = aRhythm [ 1 ]

endif	

kAmplitude jspline .1, 0, 4
kAmplitude += .1
kAmplitude = 1 - kAmplitude

aLeft clip aLeft * kAmplitude, 1, 1
aRight clip aRight * kAmplitude, 1, 1

outs aLeft, aRight

endin

-==

#### Node.js Rhythm Scorer ES Module

?# cat - > ._/rhythm.mjs

+==

import Scenarist from '@faddys/scenarist';
import $0 from '@faddys/command';

await Scenarist ( new class Rhythm {

sequence = []
tempo = 105
length = 4
measure = 8

constructor () {

Object .keys ( this )
.filter ( key => typeof this [ key ] === 'number' )
.forEach ( key => Object .defineProperty ( this, '$' + key, {

value ( $, value, ... argv ) {

this [ key ] = parseFloat ( value ) || this [ key ];

return $ ( ... argv );

}

} ) );

}

async $_producer ( $ ) {

await $ ( ... process .argv .slice ( 2 ) );

const rhythm = this;

console .log ( `

${ Object .values ( rhythm .kit ) .join ( '\n' ) }

t 0 ${ rhythm .tempo }
v ${ rhythm .length }

${ rhythm .sequence .join ( '\n' ) }

e 60

` .trim () );

}

async $_director ( $, ... argv ) {

if ( ! argv .length )
return;

const rhythm = this;
const [ step, kit ] = argv .shift () .split ( '/' );

rhythm .sequence .push ( [

'i "rhythm"',
`[${ step }/${ rhythm .measure }]`,
-1,
`"${ kit }"`

] .join ( ' ' ) );

await rhythm .prepare ( kit )

await $ ( ... argv );

}

kit = {};

async prepare ( kit ) {

const rhythm = this;

if ( rhythm .kit [ kit ] )
return;

const contents = await $0 ( 'file', '--mime-type', kit + '/*' )
.then ( $ => $ ( Symbol .for ( 'output' ) ) );

rhythm .kit [ kit ] = contents .map ( line => line .split ( /\s+/ ) )
.filter ( ( [ file, type ] ) => type .startsWith ( 'audio' ) )
.map ( ( [ file ] ) => `i "kit" 0 0 "${ kit }" "../${ file .slice ( 0, -1 ) }"` )
.join ( '\n' );

}

} );

-==

?# $ node ._/rhythm.mjs > ._/index.sco

?# cd ._ ; csound -odac index.* -b 384 -B 1024
