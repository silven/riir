"use strict";

const rust = require( './target/wasm32-unknown-unknown/release/wasm.js' );

const some_value = rust.back_and_forth();
const hash = rust.sha1( "" + some_value );

console.log( "The has is '" + hash + "'" );
