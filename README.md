# dns_client

This is a simple DNS client

# Setup dev environment
We use Docker container as dev and test envrionment, so you need to:
* Build the rustbuildserver image with:

```
docker build -t rustbuildserver .
```
* Then you can build and run tests with scripts/build.sh and scripts/run.sh

## Todo
* generate_id for each query, and match it with response
* support IPv6
* multi-thread
* caching
* timeout handling, retransmission
* TcpFallback
* fix all "todo" tags
* fix all warnings
* fix all 'expect's and 'unwrap's
* fix all 'panic!'s
* testcases
* check when to use &, &mut, moving
* get list of interfaces
* TCP support
* parse OPT record in response
* DNS ResourceRecord should be implemented as Enum. Currently, I don't know how to handle  
customized resouce record like OPT for EDNS
* Async network
## Done
* read configuration from file (or something else), rather than hardcode
* setting DNS servers