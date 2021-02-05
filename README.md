# Demo for Rust I - KX

[find the Slides here](https://docs.google.com/presentation/d/1xIWN236ETHL7K8rPobAsBVkocSIIkKm1Bq1dF5796-g/edit?usp=sharing)

## docs-test demo

```sh
cd demo-doc-tests
cargo test
```

## node fib rust native extension demo

```sh
# build the rust lib
cd demo-node-fib
cargo build --relase

# install node packages
cd node-show
npm i -g node-gyp
npm i

# run the examples
time node fib-node-native.js
time node fib.js
```