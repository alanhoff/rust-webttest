# WebTTest

A local WebTransport echo server.

### Instructions

Run it with `cargo run`, wait for the certificate fingerprint to appear. Open the latest Chrome version using `--ignore-certificate-errors-spki-list=FINGERPRINT --origin-to-force-quic-on=localhost:4433` and replace `FINGERPRINT` with the one that appeared on your terminal.

Go to [webtransport.day](https://webtransport.day) and use `https://localhost:4433` as the connection URL.
