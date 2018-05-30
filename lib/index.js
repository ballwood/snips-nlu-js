var NluEngine = require('../native').NluEngine;

module.exports = function (filePath) {

  // todo: actually make this async ;'(
  return new NluEngine(filePath);

};
