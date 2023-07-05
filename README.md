# Rust code coverage

Code coverage is a metric that can help you understand how much of your source is tested. It's a very useful metric that can help you assess the quality of your test suite, and we will see here how you can get started with your projects. 

Code coverage tools will use one or more criteria to determine how your code was exercised or not during the execution of your test suite. The common metrics that you might see mentioned in your coverage reports include:
- **Function coverage**: how many of the functions defined have been called.
- **Statement coverage**: how many of the statements in the program have been executed.
- **Branches coverage**: how many of the branches of the control structures (if statements for instance) have been executed.
- **Condition coverage**: how many of the boolean sub-expressions have been tested for a true and a false value.
- **Line coverage**: how many of lines of source code have been tested.


### How to do it in **rust**?

In this folder we have simple rust library with 4 functions, so how can we measure our code coverage. There are multiple way to measure code coverage in rust, here we will focus on following 2:
- [grcov](https://github.com/mozilla/grcov), tool developed by mozilla to measure code coverage
- [tarpaulin](https://github.com/xd009642/tarpaulin), code coverage reporting tool developed by [xd009642](https://github.com/xd009642)



### [grcov](https://github.com/mozilla/grcov)

To install `grcov` we need to have `rust` and `cargo` installed, then we just need to run:
```sh
cargo install grcov
```

To get code coverage using grcov, we first must add `llvm-tools` to rust
```sh
rustup component add llvm-tools-preview
```

Then we need to create `.gcno`, these files contain basic information to reconstruct basic block graphs and assign source line numbers to those blocks, we can generate these by running:
```sh
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' cargo build
```

After build runs, we need to generate `.gcda` files, they contain information of which code is called and how many times, this is needed so we can see what code is used in tests and is covered
```sh
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
```

We can see many `.profraw` files generated, we need this to generate code coverage, we can do that using `grcov` if you already have report generated we need to delete it first
```sh
rm -rf coverage/
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o ./coverage/html
```

This should generate folder `coverage/html` in it should be `index.html` we can open this browser and see report of code coverage, it should contain lines, functions and branches, `grcov` has options to export into other formats than `html`, you can get `lcov` output by passing `-t lcov`, or `coveralls` output by passing `-t coveralls --token YOUR_COVERALLS_TOKEN` with your coveralls token

After report is generated we need to clean up `.profraw` files we can do that by running:
```sh
rm `find ./ | grep .profraw`
```

All commands above are in [`code_coverage.sh`](https://raw.githubusercontent.com/0xMimir/rust-code-coverage/d2df510b55c0f466dd3a4c4a456b3a809e2f2405/code_coverage.sh) file,


### [tarpaulin](https://github.com/xd009642/tarpaulin)

Tarpaulin does everything above and more in one package, to install it you will need `cargo` and `rust` installed:
```sh
cargo install cargo-tarpaulin
```

If you are using mac, there migth be problems with `libgit2` to fix them uninstall `libgit2` and install it using `brew` or from source, **DO NOT** use macports


After that just run
```sh
cargo tarpaulin
```

Tarpaulin will automaticly run all tests and generate all files it need after that it will print out code coverage:
```sh
|| Uncovered Lines:
|| src/lib.rs: 5-6, 9-10, 13-14
|| Tested/Total Lines:
|| src/lib.rs: 2/8
|| 
25.00% coverage, 2/8 lines covered
```

Tarpaulin also support multiple outputs like `json`, `xml`, `html`, and `lcov`. To get output in one of these formats pass `-o` flag
```sh
cargo tarpaulin -o Json
cargo tarpaulin -o Xml
cargo tarpaulin -o Html
cargo tarpaulin -o Lcov
```


### Gitlab

How to add code coverage calculating to gitlab pipeline, since tests must be ran during code coverage analysis, this stage in pipeline can be merged with test stage, to your `.gitlab-ci.yml` add following:
```yml
code-coverage-and-tests:
  stage: prepare
  tags:
    - docker-executor
  image: rust
  before_script:
    - cargo install cargo-tarpaulin
  script:
    - cargo tarpaulin
  coverage: '/^\d+.\d+% coverage/'
  allow_failure: false
```

Contents:
- `code-coverage-and-tests` is name of the job, it can be named anything you want, 
- `stage` is name of stage in which job belongs
- `tags` is tag of runner
- `image` is name of docker image which is used to run job, this can be changed, but image either has to have cargo and rust installed or you have to install it in `before_script`
- `before_script` what commands to run before `script` since cargo is installed in `rust` image, we only have to install `cargo-tarpaulin`
- `script` is script to run, we only need to run `cargo tarpaulin`, if you want to save output as json or html, this should be changed
- `coverage` here we pass regex, that should detect coverage this is specific for tarpaulin and would you can see what should be used for different tools [here](https://docs.gitlab.com/ee/ci/testing/code_coverage.html)
- `allow_failure` allow for `script` to fail