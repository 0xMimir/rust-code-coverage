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

### Complex testing

In a project you will have multiple tests, unit tests, integration tests, and end-to-end tests so getting whole projects code coverage is more complex, in most cases you don't want to run some of those tests in actual enviroment, you want to run them in clean env, to test if project sets itself up correctly, if migrations are runned correctly and so on. So some of those tests will have `#[ignore]` tag on them. But in the pipeline on github or gitlab, you want to run these test and include them in code coverage.

If you take a look at [`lib.rs`](https://github.com/0xMimir/rust-code-coverage/blob/d2df510b55c0f466dd3a4c4a456b3a809e2f2405/src/lib.rs) you can see there are 3 tests, for 3 different functions, but 2 of them are ignored

So when we run `cargo tarpaulin` we get:
```sh
|| Uncovered Lines:
|| src/lib.rs: 5-6, 9-10, 13-14
|| Tested/Total Lines:
|| src/lib.rs: 2/8 -50.00%
|| 
25.00% coverage, 2/8 lines covered, -50.00% change in coverage
```

Which is correct, we only cover 1/4 functions with default tests, but when we run `cargo tarpaulin --ignored` we get following output: 
```sh
Jul 10 10:28:40.242  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/lib.rs: 9-10
|| Tested/Total Lines:
|| src/lib.rs: 6/8 +0.00%
|| 
75.00% coverage, 6/8 lines covered, +0.00% change in coverage
```

when, `--ignored` flag is passed tarpaulin runs both defualt and ignored tests which gives us out full test coverage of 75% or 3/4 test. Now lets say `subtract_test` is special test, that only must be ran on empty DB, then we can't just run `cargo tarpaulin --ignored`, we must run tests in order
```sh
cargo test -- --ignored tests::subtract_test
cargo test -- --ignored tests::divide_test
cargo test
```

So this is how we should run tests in order to see if everything is working correctly but how can we get code coverage.

```sh
cargo tarpaulin -- --ignored tests::subtract_test
cargo tarpaulin -- --ignored tests::divide_test
cargo tarpaulin
```

If we run commands above we get following:
```sh
...
|| Uncovered Lines:
|| src/lib.rs: 1-2, 9-10, 13-14
|| Tested/Total Lines:
|| src/lib.rs: 2/8
|| 
25.00% coverage, 2/8 lines covered
...
|| Uncovered Lines:
|| src/lib.rs: 1-2, 5-6, 9-10
|| Tested/Total Lines:
|| src/lib.rs: 2/8 +0.00%
|| 
25.00% coverage, 2/8 lines covered, +0.00% change in coverage
...
|| Uncovered Lines:
|| src/lib.rs: 5-6, 9-10, 13-14
|| Tested/Total Lines:
|| src/lib.rs: 2/8 +0.00%
|| 
25.00% coverage, 2/8 lines covered, +0.00% change in coverage
```

Now we only get partial test coverages, insted of whole test coverage. If we wanted to get whole test coverage using tarpaulin we would need to create config file, for example above config would look like 
```toml
[subtract_test]
args = ["--ignored", "divide_test"]

[divide_test]
args = ["--ignored", "subtract_test"]

[test]
```

After that we just need to run:
```sh
cargo tarpaulin --config config.toml
```

Output from following command looks like
```sh
...
running 1 test
test tests::divide_test ... ok
...
running 1 test
test tests::subtract_test ... ok
...
running 3 tests
test tests::add_test ... ok
test tests::divide_test ... ignored
test tests::subtract_test ... ignored
...
|| Uncovered Lines:
|| src/lib.rs: 9-10
|| Tested/Total Lines:
|| src/lib.rs: 6/8 +0.00%
|| 
75.00% coverage, 6/8 lines covered, +0.00% change in coverage
```

When using config each of tests is ran in order with args we define, and at the end all of results are combined in single report, this is very useful, both as testing tool and code coverage tool