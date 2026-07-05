# Testing Commands

## pytest

```bash
$ miskin pytest

PASSED: 23 tests
```

Failures — shows only failed test names and traceback snippets:

```bash
$ miskin pytest

FAILED: 2/23

test_auth_login | AssertionError: expected 200 got 401
test_profile_update | assert None is not None
```

## jest / vitest

```bash
$ miskin jest

PASSED: 15/15 tests (5 suites)
```

```bash
$ miskin vitest

FAILED: 2/15
FAIL  src/__tests__/auth.test.ts
FAIL  src/__tests__/profile.test.ts
```

## go test

```bash
$ miskin go test

ok  3 packages
```

With failures:

```bash
3 packages, 1 failed
--- FAIL: TestHandler (0.01s)
```

## rspec

```bash
$ miskin rspec

Finished in 0.123 seconds
3 examples, 0 failures
```

## playwright

```bash
$ miskin playwright test

PASSED: 12 tests
```

## Generic Test Runner

Any unrecognized test runner gets basic filtering:

```bash
$ miskin unknown-runner test

ok          # exit code 0
FAIL: ...   # exit code != 0, shows error lines
```
