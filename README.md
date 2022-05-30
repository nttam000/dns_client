# dns_client

This is a simple DNS client

## Design

### Mod
* Api (root lib)
* Core
* Parser
* QueryHandler
* UdpController

### Flow

Api
 |
Core
 |
QueryHandler (thread pool)
 |
UdpController (singleton)

## Todo
* multi-thread
* caching
* timeout handling, retransmission
* TcpFallback
* fix all "todo" tags
* fix all warnings
* fix all 'expect's and 'unwrap's
* testcases
* check when to use &, &mut, moving
* get list of interfaces
* setting DNS servers