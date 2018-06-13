# snips-nlu-js

Quick and dirty NodeJS bindings for snips-nlu-rs.

Work in progress
----------------------

- Library needs to be made async, loading and parsing block, will have to work out how to do it in Neon.
- Implement filter_intents parameter
- Unit tests
- Zip/Dir loading
- Feature parity with other FFI's

Installing
----------------------

#### Download Rust
```shell
curl https://sh.rustup.rs -sSf | sh
```

#### Install
```shell
npm install
```

Building for AWS Lambda
----------------------

#### Prerequisite
Set dockers max container memory to 4gb. snips-nlu-rs uses a lot when compiling. See the issue [here](https://github.com/snipsco/snips-nlu-rs/issues/22#issuecomment-396869682) 

#### Build docker container
```shell
docker build -t snips-nlu-js .
```

#### Run docker container
```shell
docker run -t -i snips-nlu-js
```
You may also want to mount a local filesystem to the directory /work so you can extract the files from the container with the `-v /host/directory:/work` command

#### Build
```shell
npm install
```

Usage
----------------------
Currently only local usage only, binaries will be published on release

```shell
cd path/to/snips-nlu-js
npm link
```

```shell
cd path/to/your/project
npm link snips-nlu-js
```

In your code:
```javascript
const createNluEngine = require('snips-nlu-js');

try {
  const nluEngine = createNluEngine('./path/to/assistant.json');
  const result = nluEngine.parse('your string');
  console.log(result);
} catch (e) {
  console.error(e.message);
}


```

Demo
----------------------

#### Cli
```shell
./cli.js trained_assistant.json weather
```

Node Version Support
----------

The package has been developed on Node 8 and it worked on Node 6. Enough for AWS Lambda.

Author
------

[Barry Allwood](https://github.com/ballwood)

Thanks to the [Neon](https://github.com/neon-bindings/neon) project for their rust bindings
and [Snips](https://github.com/snipsco/) for their super fast [rust implementation](https://github.com/snipsco/snips-nlu-rs/) of their nlu engine 

License
-------

Licensed under the Apache License, Version 2.0.
