desc
====

This is a tool to manipulate `DESC` files in Rfam. This is meant to make it
easy to script edits to a large collection of `DESC` files.

## Usage

The usage is:

```sh
$ desc <options> <filename> <command> <args>
```

Generally the `filename` is `DESC`.

This has some help docs which can be see with `--help`, eg:

```sh
$ desc --help
```

Individual commands have help as well with `--help` or `help <command>`.

## Examples

Here a are a few examples of

### Adding references

Add a database cross reference to a [`GO`](https://www.ebi.ac.uk/QuickGO/) term
in the `DESC` file.

```sh
$ desc DESC add GO:0016442
```

Add a cross reference to a URL to the DESC file.

```sh
$ desc DESC add https://example.org
```

### Removing references

Remove a database cross reference.

```sh
$ desc DESC remove GO:0016442
```

Remove a publication.

```sh
$ desc DESC remove PMID:1
```

Remove an author from the file.

```sh
$ desc DESC remove ORCID:1
$ grep -c ORCID:1 DESC
0
```

## Useful Links

- [Rfam help](https://docs.rfam.org/)
