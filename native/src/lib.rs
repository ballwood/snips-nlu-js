#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate snips_nlu_lib;

use snips_nlu_lib::{
  FileBasedConfiguration,
  SnipsNluEngine
};

use neon::js::{
  Object,
  JsString,
  JsFunction
};

use neon::mem::Handle;
use neon::vm::Lock;

use neon::js::class::{Class, JsClass};

pub struct NluEngine {
  engine: SnipsNluEngine
}

declare_types! {
  pub class JsNluEngine for NluEngine {
    init(call) {
      let scope = call.scope;

      let model_file = call.arguments.require(scope, 0)?.check::<JsString>()?;

      let configuration = match FileBasedConfiguration::from_path(model_file.value(), false) {
        Ok(conf) => conf,
        Err(_e) => panic!(format!("{}", _e)),
      };

      let _nlu_engine = match SnipsNluEngine::new(configuration) {
        Ok(engine) => engine,
        Err(_e) => panic!(format!("{}", _e))
      };

      Ok(NluEngine {
        engine: _nlu_engine
      })
    }

    method parse(call) {

      let scope = call.scope;

      let _query_str = match call.arguments.require(scope, 0)?.check::<JsString>() {
        Ok(_str) => _str.value(),
        Err(_e) => panic!("parse: argument 0 (query_str) is not of type string")
      };

      // todo: implement filter intents
      // let filter_intents_vector = match call.arguments.require(scope, 1)?.check::<JsArray>() {
      //    Ok(filter_intents) => filter_intents.to_vec(scope).unwrap(),
      //    Err(e) => panic!("parse: argument 1 (intents_filter) is not of type array")
      // };

      let _nlp_result = call.arguments.this(scope).grab(|nlpengine| { nlpengine.engine.parse(&_query_str, None) }).unwrap();

      Ok(neon_serde::to_value(scope, &_nlp_result)?)

    }
  }
}

register_module!(m, {
  let class: Handle<JsClass<JsNluEngine>> = JsNluEngine::class(m.scope)?;
  let constructor: Handle<JsFunction<JsNluEngine>> = class.constructor(m.scope)?;
  m.exports.set("NluEngine", constructor)?;
  Ok(())
});
