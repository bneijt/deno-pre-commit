# deno-pre-commit

[pre-commit](https://pre-commit.com/) hook for [Deno](https://deno.com/).

Supported hook ids are:

- `deno-fmt`: Format files, `deno fmt`.
- `deno-lint`: Lint and fix, `deno lint --fix`.
- `deno-fmt-check`: Check formatting, `deno fmt --check`.
- `deno-lint-check`: Check linting, `deno lint`.

Make sure you pick a version with the `rev: ` attribute. These point to the corresponding `deno` version on [npmjs](https://www.npmjs.com/package/deno)

## Usage

Update your `.pre-commit-config.yaml` with one of the following snippets. See at the bottom for a full example.

Simple "_fix everything you see_" hook config is:
```
repos:
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 1.43.5
    hooks:
      - id: deno-fmt
      - id: deno-lint
```

You can also opt for the "_only check, don't change_" approach:
```
repos:
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 1.43.5
    hooks:
      - id: deno-fmt-check
      - id: deno-lint-check
```

If you are using [Ruff](https://docs.astral.sh/ruff/), you probably already have your Jupyter notebooks
formatted.

To skip formatting with deno during pre-commit, update your pre-commit config to exclude _jupyter_ type files. Below is a snippet with the `exclude_types` attribute added to the `deno-fmt` hook.

```
repos:
  - repo: https://github.com/bneijt/deno-pre-commit
    rev: 1.43.5
    hooks:
      - id: deno-fmt
        exclude_types: ["jupyter"]
```

## Contributing
If you would like to see support for more hooks with `deno`, consider filing an issue.
Or better yet, proposing a PR.
