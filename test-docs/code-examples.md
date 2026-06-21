# Code Block Showcase

← [Back to Home](../README.md)

Exercises the syntax highlighter and the language label across a range of languages.

## Systems

```rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

```c
#include <stdio.h>

int main(void) {
    for (int i = 0; i < 10; i++) {
        printf("%d\n", i);
    }
    return 0;
}
```

```go
package main

import "fmt"

func main() {
    ch := make(chan int, 5)
    go func() {
        for i := range ch {
            fmt.Println(i)
        }
    }()
    for i := 0; i < 5; i++ {
        ch <- i
    }
    close(ch)
}
```

## Scripting

```python
from dataclasses import dataclass
from typing import Iterator

@dataclass
class Range:
    start: int
    stop: int

    def __iter__(self) -> Iterator[int]:
        return iter(range(self.start, self.stop))

for n in Range(1, 6):
    print(n ** 2)
```

```javascript
const debounce = (fn, ms) => {
  let timer;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
};
```

```typescript
type Result<T, E> =
  | { ok: true; value: T }
  | { ok: false; error: E };

function divide(a: number, b: number): Result<number, string> {
  if (b === 0) return { ok: false, error: "division by zero" };
  return { ok: true, value: a / b };
}
```

```ruby
class Stack
  def initialize = (@data = [])
  def push(v) = @data.push(v)
  def pop = @data.pop
  def peek = @data.last
  def empty? = @data.empty?
end
```

## Web

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Example</title>
</head>
<body>
  <h1>Hello, world!</h1>
</body>
</html>
```

```css
:root {
  --color-bg: #ffffff;
  --color-text: #24292f;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #0d1117;
    --color-text: #e6edf3;
  }
}

body {
  background: var(--color-bg);
  color: var(--color-text);
}
```

## Data & Config

```json
{
  "name": "mdpreview",
  "version": "0.1.0",
  "features": ["live-reload", "mermaid", "syntax-highlighting"],
  "port": 3000
}
```

```toml
[package]
name = "mdpreview"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.8", features = ["ws"] }
comrak = { version = "0.39", features = ["syntect"] }
tokio = { version = "1", features = ["full"] }
```

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test
```

```sql
SELECT
    u.name,
    COUNT(p.id)  AS post_count,
    MAX(p.created_at) AS last_post
FROM users u
LEFT JOIN posts p ON p.user_id = u.id
WHERE u.active = true
GROUP BY u.id, u.name
ORDER BY post_count DESC
LIMIT 10;
```

## Infra

```bash
#!/usr/bin/env bash
set -euo pipefail

VERSION=$(cargo metadata --no-deps --format-version 1 \
  | jq -r '.packages[0].version')

echo "Building mdpreview v$VERSION"
cargo build --release
strip target/release/mdpreview
```

```dockerfile
FROM rust:1.78-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mdpreview /usr/local/bin/
ENTRYPOINT ["mdpreview"]
```
