# foreground cron
Cron implementation that runs jobs from crontab-file in foreground.

## Install
### shell
Downloads latest release for linux x86_64 to ~/.bin/ and makes it executable.
```sh
curl https://raw.githubusercontent.com/arve0/foreground-cron/master/install.sh | sh
```

### cargo
```bash
cargo install foreground-cron
```

### download
[Download the static binary for linux x86_64](https://github.com/arve0/foreground-cron/releases) in relases.

## Usage
```bash
echo "* * * * * date" > crontab
foreground-crontab
```

Reads the file `crontab` and runs jobs at their specified schedule.

## Example:
This `crontab`-file (lines starting with `#` are ignored)
```crontab
# min hour day month dayofweek cmd
  *   *    *   *     *         ls -l crontab
```

will print

```
-rw-r--r--  1 user  user  77 Mar 20 16:28 crontab
```

every minute.

## Development
```bash
git clone https://github.com/arve0/foreground-cron
cd foreground-cron
cargo install cargo-watch
cargo watch -x check -x test -x run
```

#### Building static binary
Requires Docker installed.
```bash
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```
For more details, see https://github.com/emk/rust-musl-builder.
