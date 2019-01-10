# AdEx Network validator implementation [![Build Status](https://travis-ci.org/elpiel/adex-validator.svg?branch=master)](https://travis-ci.org/elpiel/adex-validator) [![codecov](https://codecov.io/gh/elpiel/adex-validator/branch/master/graph/badge.svg)](https://codecov.io/gh/elpiel/adex-validator)

This is a repository implementing the `get_healt` and `is_valid_transaction` as an assignment from [AdEx Network](https://www.adex.network/) .

## JS Implementation provided for this assignment
*NOTE: In the assignment was mentioned that the `get_health` implementation is missing the `min` implementation,
so both the code and the tests in the code examples are wrong.*
 
* [Implementation in adex-validator-stack-js repository](https://github.com/AdExNetwork/adex-validator-stack-js/blob/master/services/validatorWorker/lib/followerRules.js)
* [Tests in adex-validator-stack-js repository](https://github.com/AdExNetwork/adex-validator-stack-js/blob/master/test/index.js)

## Running the code

You can run the tests with cargo:

`cargo test`