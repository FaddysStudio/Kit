#!/usr/bin/env roll

# Faddy's Kit Beat

?# cat - > .FaddysKit/beat.mjs

+==
import Scenarist from '@faddys/scenarist';

await Scenarist ( new class Beat {

bank = '../.FaddysBank';

$bank ( $, value, ... argv ) {

this .bank = value;

return $ ( ... argv );

}

track = "track-" + Date .now ()

$track ( $, value, ... argv ) {

this .track = value;

return $ ( ... argv );

}

instrument = 13
tempo = 110;

$tempo ( $, value, ... argv ) {

this .tempo = parseFloat ( value ) || this .tempo;

return $ ( ... argv );

}

length = 4
measure = 8
sequence = []

async $_producer ( $ ) {

await $ ( ... process .argv .slice ( 2 ) );

const beat = this;

console .log ( `

t 0 ${ beat .tempo }
v ${ beat .length }

${ beat .sequence .join ( '\n' ) }

s; ${ beat .length }

` .trim () );

}

async $_director ( $, ... argv ) {

if ( ! argv .length )
return '';

const beat = this;
const [ step, sample ] = argv .shift () .split ( '/' );

beat .sequence .push ( `i ${ beat .instrument }.${ beat .sequence .length } [${ step }/${ beat .measure }] 1 "${ beat .track }" "${ beat .bank }/${ sample }.wav"` );

await $ ( ... argv );

}

} );
-==

?# $ node .FaddysBeat/index.mjs
