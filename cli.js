#!/usr/bin/env node

const createNluEngine = require('./lib/index.js');
const path = require('path');

const file = process.argv[2];

const nluEngine = createNluEngine(file);

console.log(nluEngine.parse(process.argv[3]));
