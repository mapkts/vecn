# Changelog

## [1.0.0](https://www.github.com/mapkts/vecn/compare/v0.1.0...v1.0.0) (2021-04-30)


### ⚠ BREAKING CHANGES

* change `Op<Rhs>` to only `Op<Self>`

### Features

* add a cross method ([96489dd](https://www.github.com/mapkts/vecn/commit/96489dd7d69eae676c382e192e1945d8a5cba30c))
* add fn `is_nan` ([d1ea347](https://www.github.com/mapkts/vecn/commit/d1ea347ebb631c37a602192ef59139baf146f635))


### Code Refactoring

* change `Op<Rhs>` to only `Op<Self>` ([2b2736a](https://www.github.com/mapkts/vecn/commit/2b2736adc09ac9754615bbb68d4252acd22f6093))

## 0.1.0 (2021-04-25)


### ⚠ BREAKING CHANGES

* refactor error parsing messages

### Features

* add ui tests ([5842ab0](https://www.github.com/mapkts/vecn/commit/5842ab012bb9e3040e1416939908f72715b4b6ed))
* add unit tests ([2a6148b](https://www.github.com/mapkts/vecn/commit/2a6148bea08ed3a4d0c06ab708d77da53c3dd8f4))


### Bug Fixes

* don't implement `normalize` on struct with more than 4 fields ([97f7566](https://www.github.com/mapkts/vecn/commit/97f756665c68bbd7f60408370499c6f6f50cea86))
* fix broken Cargo.toml ([daba63c](https://www.github.com/mapkts/vecn/commit/daba63c802049ebd1a447eb800034b7bcfe56f59))
* replace `reduce` method call with for loop ([f5e4cda](https://www.github.com/mapkts/vecn/commit/f5e4cdaad9bac90cce0cadb1525d5b233cae82d3))


### Code Refactoring

* refactor error parsing messages ([2ace380](https://www.github.com/mapkts/vecn/commit/2ace3800a4be041124e14929fb515f075695ec4a))
