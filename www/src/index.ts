import { message } from 'wasm-example'

import { Something } from './something/something'

let something = new Something("text")

message("Something: " + something.getText())
