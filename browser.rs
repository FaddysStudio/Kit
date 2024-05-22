#!/usr/bin/env roll

# Faddy's Kit Browser

This roll is responsible for browsing the directory passed as the last argument,
and listing all the audio file found inside.

## Syntax

```bash
roll browser.rs [ ... options ] <DIRECTORY>
```

/?# mkdir -p .FaddysKit ; cd .FaddysKit ; npm i @faddys/scenarist

?# cat - > .FaddysKit/browser.mjs

+==
import Scenarist from '@faddys/scenarist';
import { createInterface } from 'node:readline';
import { stdin as input } from 'node:process';

await Scenarist ( new class {

$_producer ( $ ) {

$ ( ... process .argv .slice ( 2 ) );

}

$mimeTypes ( $, directory ) {

console .log ( `file --mime-type ${ directory }/*` );

}

$audioFiles () {

createInterface ( { input } )
.on ( 'line', line => {

const [ file, type ] = line .split ( /\s+/ );

if ( type .startsWith ( 'audio' ) )
console .log ( file .slice ( 0, -1 ) );

} );

}

} );
-==

?# $ node .FaddysKit/browser.mjs mimeTypes > .FaddysKit/browser.sh
?# bash .FaddysKit/browser.sh | node .FaddysKit/browser.mjs audioFiles
