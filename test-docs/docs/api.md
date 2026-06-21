# API Reference

## CLI Options

```
mdpreview [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to a .md file or directory [default: current directory]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Live Reload Protocol

The server exposes a WebSocket endpoint at `/ws`. When any `.md` file changes, the server broadcasts:

```json
{"type": "reload"}
```

The client then fetches the updated content from `/render?path=<current-file>` and swaps the content inline without a full page reload.
