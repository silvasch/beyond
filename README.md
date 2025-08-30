# beyond

Easily execute rust functions on an external machine instead of locally.

`beyond` achieves this by generating the code for a client and a server
that communicate over SSH. This has multiple advantages over using other
protocols like HTTP:

- Because SSH already implements a mechanism for authentication, `beyond`
  gets security by default.
- Almost every remote machine already exposes SSH to the network - no need
  for allowing a port in your firewall or even setting up a reverse proxy.

## Usage

Instead of writing multiple paragraphs about how to use this crate,
I will instead redirect you to the example in `crates/beyond_example`.
It contains comments explaining every part of the code.

