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

?# mkdir -p .FaddysKit

### Csound Engine

#### Csound Orchestra Index

?# cat - > .FaddysKit/index.orc

+==

sr = 44100
ksmps = 32
nchnls = 2
0dbfs = 1

#include "kit.orc"
#include "rhythm.orc"
#include "beat.orc"
#include "recorder.orc"
#include "out.orc"
#include "loop.orc"

-==

#### Csound Kit Instrument

?# cat - > .FaddysKit/kit.orc

+==

instr kit

SKit strget p4
SPath strget p5

iReady chnget SPath

if iReady == 0 then

SSize sprintf "%s/size", SKit
iSize chnget SSize

SIndex sprintf "%s/%d", SKit, iSize

chnset SPath, SIndex
chnset iSize + 1, SSize

chnset 1, SPath

endif

endin

-==

### Csound Recorder Instrument

?# cat - > .FaddysKit/recorder.orc

+==

instr recorder

aInputLeft, aInputRight ins

STitle strget p4

SInput sprintf "%s_input.wav", STitle

fout SInput, -1, aInputLeft, aInputRight

aRhythmLeft chnget "left"
aRhythmRight chnget "right"

SNote sprintf "%s_rhythm.wav", STitle

fout SNote, -1, aRhythmLeft, aRhythmRight

SMix sprintf "%s.wav", STitle

aLeft clip aInputLeft + aRhythmLeft, 1, 1
aRight clip aInputRight + aRhythmRight, 1, 1

fout SMix, -1, aLeft, aRight

endin

-==

## Csound Rhythm Instrument

This instrument plays one of the beats passed as string p-fields once per cycle.
A cycle is the absolute value of p3.

```csound
i "rhythm" $delay $cycle "$beat1" ... "$beatN"
```

?# cat - > .FaddysKit/rhythm.orc

+==

instr rhythm

iSwing random 0, 127

if iSwing > p5 then

iInstance chnget "instance"
iInstance += 1
chnset iInstance, "instance"

iInstance /= 1000
p1 init int ( p1 ) + iInstance

iBeat nstrnum "beat"

SKit strget p4
SKit strget p4
SSize sprintf "%s/size", SKit
iKit chnget SSize

iIndex random 0, iKit - 1
SIndex sprintf "%s/%d", SKit, iIndex

SNote chnget SIndex

SBeat sprintf {{ i %f 0 1 "%s" }}, iBeat + iInstance, SNote

scoreline_i SBeat

endif

endin

-==

#### Csound Beat Player Instrument

?# cat - > .FaddysKit/beat.orc

+==

instr beat

SNote strget p4

p3 filelen SNote

kPitch jspline 1, 0, 4

aRhythm [] diskin2 SNote, cent ( kPitch * 10 )

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

chnmix aLeft, "left"
chnmix aRight, "right"

endin

-==

?# cat - > .FaddysKit/out.orc

+==

instr out

aLeft chnget "left"
aRight chnget "right"

chnclear "left"
chnclear "right"

aLeft clip aLeft, 1, 1
aRight clip aRight, 1, 1

out aLeft/2, aRight/2

endin

-==

?# cat - > .FaddysKit/loop.orc

+==

instr loop

rewindscore

endin

-==

#### Node.js Rhythm Scorer ES Module

?# cat - > .FaddysKit/rhythm.mjs

+==

import Scenarist from '@faddys/scenarist';
import $0 from '@faddys/command';

await Scenarist ( class Beat {

score = []
tempo = 105
length = 4
measure = 8

static async $_producer ( $ ) {

const score = [

//'i "recorder" 0 -1 "RecordingTest"',
'i "out" 0 -1',

];

const files = process .argv .slice ( 2 );

for ( let index = 0; index < files .length; index++ )
score .push ( await $ ( files [ index ], index === files .length - 1 )
.then ( $ => $ ( Symbol .for ( 'score' ) ) ) );

score .unshift ( Object .values ( Beat .kit ) .join ( '\n' ) );

console .log ( score .join ( '\n\n' ) );

}

constructor ( file, loop ) {

this .file = file;
this .loop = loop;

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

const beat = this;
const notation = await $0 ( 'cat', beat .file )
.then ( $ => $ ( Symbol .for ( 'output' ) ) );

for ( let line of notation )
if ( ( line = line .trim () ) .length )
await $ ( ... line .trim () .split ( /\s+/ ) );

}

static tempo = 0
static length = 0

$_score () {

const beat = this;

Beat .length += ( beat .length *= 60 / beat .tempo );
Beat .tempo = beat .tempo;

if ( beat .loop )
beat .score .push ( `i "loop" ${ Beat .length } 1` );

return beat .score .join ( '\n' );

}

async $_director ( $, ... argv ) {

if ( ! argv .length )
return;

const beat = this;
const [ step, kit, swing ] = argv .shift () .split ( '/' );

if ( typeof beat .start !== 'number' )
beat .start = Beat .length * Beat .tempo / beat .tempo;

beat .score .push ( [

'i "rhythm"',
Beat .length + parseFloat ( step ) / beat .measure * beat .length * 60 / beat .tempo,
0,
`"${ kit }"`,
parseFloat ( swing ) || 0

] .join ( ' ' ) );

await Beat .prepare ( kit )

await $ ( ... argv );

}

static kit = {};

static async prepare ( kit ) {

const beat = this;

if ( beat .kit [ kit ] )
return;

const contents = await $0 ( 'file', '--mime-type', kit + '/*' )
.then ( $ => $ ( Symbol .for ( 'output' ) ) );

beat .kit [ kit ] = contents .map ( line => line .split ( /\s+/ ) )
.filter ( ( [ file, type ] ) => type .startsWith ( 'audio' ) )
.map ( ( [ file ] ) => `i "kit" 0 0 "${ kit }" "../${ file .slice ( 0, -1 ) }"` )
.join ( '\n' );

}

} );

-==

?# $ node .FaddysKit/rhythm.mjs > .FaddysKit/index.sco

?# cd .FaddysKit ; csound -iadc -odac index.orc index.sco
