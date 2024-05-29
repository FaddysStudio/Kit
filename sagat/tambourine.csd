iNumberOfObjects init 4
iDamp init 5
iShakingEnergy init 6
iMainResonantFrequency init 7
i1stResonantFrequency init 8
i2ndResonantFrequency init 9

aNote tambourine kAmplitude, p3 \
p ( iNumberOfObjects ), \ ; defaults to 32
p ( iDamp ), \ ; ranges between 0 and .75
p ( iShakingEnergy ), \ ; defaults to 0, ranges between 0 and 1
p ( iMainResonantFrequency ), \ ; defaults to 2300
p ( i1stResonantFrequency ), \ ; defaults to 5600
p ( i2ndResonantFrequency ), ; defaults to 8100

aNote clip aNote, 1, 1

outs aNote, aNote

endin
