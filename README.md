# foreground cron
Cron implementation that runs jobs from crontab-file in foreground.

## Install
```bash
cargo install foreground-crontab
```

## Usage
```bash
./foreground-crontab
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