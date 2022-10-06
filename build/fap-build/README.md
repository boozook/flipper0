# Flipper Zero Application build utils

Typically for usage from build script.

It do following things:
1. Build FAP manifest using [fam crate][fam]
2. Read env var `FLIPPER_REPO_PATH`
3. Copy or link (now just link) the output dir where manifest (fam) is to `$FLIPPER_REPO_PATH/applications_user/{fap-id}/`
4. Same for assets such as icons

After this you can just execute `./fbt firmware_{fap-id}` from the root [flipperzero-firmware][] repository clone.

`$FLIPPER_REPO_PATH` should points to working copy of the flipperzero-firmware repository.


__[Example](https://github.com/boozook/flipper0/tree/master/examples/hello-fap-build/).__


[fam]: https://crates.io/crates/fam
[flipperzero-firmware]: https://github.com/flipperdevices/flipperzero-firmware
