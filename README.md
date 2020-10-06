<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://avatars1.githubusercontent.com/u/67451441?s=200&v=4" alt="Project logo"></a>
</p>

<h3 align="center">Halva Test Example</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> Halva is a toolchain for improving the experience of developing Decentralized Applications based on Substrate. 
    <br> 
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Built Using](#built_using)

## 🧐 About <a name = "about"></a>

Halva is a toolchain for developing Decentralized Applications based on Substrate. It provides a high-level way to configure a development environment, interact with Substrate through external API and writing your test cases. Halva targets testing extrinsics via RPC calls this allows test Substrate (or clients compatible with Substrate RPC) as a black-box.

## 🏁 Getting Started <a name = "getting_started"></a>


### Installing

Before running the tests, you need to:

Clone this repo

```bash
git clone https://github.com/halva-suite/halva-test-example.git

cd halva-test-example/
```

Install halva-cli

```bash
npm install halva-cli -g
```

If necessary, you can install spec-builder

```bash
npm install spec-builder -g
```


## 🔧 Running the tests <a name = "tests"></a>

All tests are in the ```test``` folder

Run node:

```bash
halva-cli start
```

Use cli to start tests:

```bash
halva-cli test
```

If you need help, use 

```bash
halva-cli --help
```

### Writing test notes

You can use global variables provided by halva

```javascript
describe('Halva test', () => {

  describe('test global', () => {
    it('Transfer balance to bob(passes)', async () => {
      const tx = halva.polkadot.tx.balances.transfer(bobPair.address, '10000000');
      await passes(tx, 'Send', alicePair);
    });
  });
});
```

##### Variables

* halva.accounts - 10 Key pairs for tests
* halva.polkadot - ApiPromise object of polkadot



## ⛏️ Built Using <a name = "built_using"></a>

- [Mocha](https://mochajs.org/) - Test framework
- [PolkadotJs](https://polkadot.js.org/) - Api for substrate
- [NodeJs](https://nodejs.org/en/) - Server Environment