# deno-pre-commit

[pre-commit](https://pre-commit.com/) hook for [Deno](https://deno.com/).

## Usage

Simple "_fix everything you see_" hook config is:
```
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 0.0.1
    hooks:
      - id: deno-fmt
      - id: deno-lint
```

You can also opt for the "_only check, don't change_" approach:
```
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 0.0.1
    hooks:
      - id: deno-fmt-check
      - id: deno-lint-check
```

If you are using `ruff` you probably already have your Jupyter notebooks
formatted.

To skip formatting of those, update your config to reflect ignoring _jupyter_:

```
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 0.0.1
    hooks:
      - id: deno-fmt
        exclude_types: ["jupyter"]
```

If you would like to see support for more hooks with `deno`, consider filing an issue.
Or better yet, proposing a PR.
