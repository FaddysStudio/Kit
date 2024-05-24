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

## Csound Orchestra Index

?# cat - > ._/index.orc

+==

sr = 48000
ksmps = 48
nchnls = 2
0dbfs = 1

#include "recorder.orc"
#include "rhythm.orc"
#include "beat.orc"

-==

## Csound Recorder Instrument

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

### Syntax for Scoring Rhythms

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

kSwing jspline .5, 0, 4
kSwing += .5

iKit pcount
iKit -= 3
SKit [] init iKit

iBeat init 0

while iBeat < iKit do

SKit [ iBeat ] strget p ( iBeat + 4 )

od

if kRhythm == 1 then

kBeat = int ( iKit * kSwing * 1.25 )

if kBeat < iKit then

SBeat = SKit [ kBeat ]

SNote sprintfk {{ i %f 0 1 "%s" }}, iBeat + iInstance, SBeat

scoreline SNote, 1

endif

endif

endin

-==

?# cat - > ._/beat.orc

+==

instr beat

SRhythm strget p4

p3 filelen SRhythm

kPitch jspline 1, 0, 4

aLeft, aRight diskin2 SRhythm, cent ( kPitch * 10 )

kAmplitude jspline .1, 0, 4
kAmplitude += .1
kAmplitude = 1 - kAmplitude

aLeft clip aLeft * kAmplitude, 1, 1
aRight clip aRight * kAmplitude, 1, 1

outs aLeft/10, aRight/10

endin

-==

?# cat - > ._/index.sco

+==

i "rhythm" 0 1 "rhythm2_90.wav"

-==

?# cd ._ ; csound -odac index.* -b 384 -B 1024
