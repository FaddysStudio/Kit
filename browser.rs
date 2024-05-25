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
import command from '@faddys/command';

await Scenarist ( new class Browser {

$_producer ( $ ) {

$ ( ... process .argv .slice ( 2 ) );

}

async $_director ( $, directory ) {

const contents = await command ( 'file', '--mime-type', directory + '/*' )
.then ( $ => $ ( Symbol .for ( 'output' ) ) );

contents .map ( line => line .split ( /\s+/ ) )
.filter ( ( [ file, type ] ) => type .startsWith ( 'audio' ) )
.forEach ( ( [ file ] ) => console .log ( 'aplay', file .slice ( 0, -1 ) ) );

}

} );
-==

?# $ node .FaddysKit/browser.mjs > .FaddysKit/response.sh
?# -1 -2 bash .FaddysKit/response.sh

/?# $ node .FaddysKit/browser.mjs mimeTypes > .FaddysKit/browser.sh
/?# bash .FaddysKit/browser.sh | node .FaddysKit/browser.mjs audioFiles
