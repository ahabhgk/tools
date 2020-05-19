# `noDuplicateKeys.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-compiler/lint/rules/js/noDuplicateKeys.test.ts --update-snapshots` to update.

## `no duplicate keys`

### `0`

```

 unknown:2:2 lint/js/noDuplicateKeys ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Duplicate key test

    1 │ const foo = {
  > 2 │   test: true,
      │   ^^^^
    3 │   test2: true,
    4 │   test: false,

  ℹ Defined already here

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```

### `0: formatted`

```
const foo = {
	test: true,
	test2: true,
	test: false,
};

```