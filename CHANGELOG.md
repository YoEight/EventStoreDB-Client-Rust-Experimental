# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed
- Rename StreamPosition::Point to StreamPosition::Position. [EventStoreDB-Client-Rust#76](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/76)
- Migrate to Rust 2021 edition. [EventStoreDB-Client-Rust#94](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/94)
- Improve integration testing. [EventStoreDB-Client-Rust#95](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/95)
- Remove `StreamName` and `GroupName` generic types. [EventStoreDB-Client-Rust#107](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/107)
- Improve streaming API for read operations. [EventStoreDB-Client-Rust#110](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/110)
- Refactor node discovery process. [EventStoreDB-Client-Rust#113](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/113)
- Expose projection-related options. [EventStoreDB-Client-Rust#117](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/117)
- Improve `append_to_stream` usability. [EventStoreDB-Client-Rust#116](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/116)

### Fixed
- Fix automatic reconnection process for stream operations. [EventStoreDB-Client-Rust#56](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/56)
- Improve server-side error management. [EventStoreDB-Client-Rust#74](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/74)
- Fix test flakeyness. [EventStoreDB-Client-Rust#85](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/85)
- Introduce more randomness when selecting a node based upon preference. [EventStoreDB-Client-Rust#88](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/88)

### Added
- Implement keep-alive [EventStoreDB-Client-Rust#53](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/53)
- Add single node tests using testcontainers. [EventStoreDB-Client-Rust#57](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/57)
- Implement auto-resubscribe when connection is dropped. [EventStoreDB-Client-Rust#58](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/58)
- Implement stream metadata operations. [EventStoreDB-Client-Rust#62](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/62)
- Implement maximum discover attempts for single node connections. [EventStoreDB-Client-Rust#67](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/67)
- Implement projection API. [EventStoreDB-Client-Rust#60](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/60)
- Implement batch-append API. [EventStoreDB-Client-Rust#77](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/77)
- Add first-class support for stream deleted error. [EventStoreDB-Client-Rust#89](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/89)
- Add timeouts when running projection tests. [EventStoreDB-Client-Rust#91](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/91)
- Send feedback to user when `batch_append` is not supported by the server. [EventStoreDB-Client-Rust#93](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/93)
- Implement Persistent Subscription to `$all`. [EventStoreDB-Client-Rust#98](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/98)
- Implement server features detection. [EventStoreDB-Client-Rust#118](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/118)
- Allow passing an external Tokio runtime when creating a new client. [EventStoreDB-Client-Rust#124](https://github.com/EventStore/EventStoreDB-Client-Rust/pull/124)

## Previous versions
1.0.0
======
* Support Tokio 1.* and Tonic 0.4.
* Fix how is_json is extracted for persistent subscriptions.
* Apply same public API as other official gRPC clients.

0.9.9
=====
* Fix how we set RecordedEvent is_json.

0.9.8
=====
* Fix insecure mode regression.

0.9.7
=====
* Update DNS discovery, so it works properly with EventStore Cloud.
* Expose `ClientSettings` values.
* Load native certificates when TLS is enabled.

0.9.6
=====
* Add Debug instance for `SubEvent`.
* No longer implicitly use `PersistentSubscriptionSettings` default values on both persistent subscription creation and update.

0.9.5
=====
* Expose subscription lifecyle events.
* Improve FilterConf usability.

0.9.4
=====
* Use consistent names across all official clients.
* Fix documentation.

0.9.3
=====
* Add prelude module.
* Re-export `FilterConf`.

0.9.2
=====
* Add DNS record type selection in connection string. DNS A queries are done by default now. Use to be SRV.
* Export `ConnectionSettingsParseError` type.
* `read_stream` returns `ReadResult::StreamNotFound` explicitly.
* Add `Position::end()`

0.9.1
=====
* Add more documentation.
* More flexible `EventData::json` and `EventData::binary`.

0.9
=====
* gRPC is now the only supported interface.
* Delete TCP interface.
* Add `read_through` function when reading a stream (`$all` included).
* Support latest `message_timeout_ms` and `checkpoint_after_ms` persistent.proto changes.
* Improve user experience.
* Implement cluster mode connection.

0.8.1
=====
* Bump protobuf version.
* Bump tonic version.
* Make Connection cloneable.
* Support DNS discovery for the TCP API.

0.8.0
=====
* Implement secure connection for the TCP API.
* Support 20.6 stable version (behind 20.6).

0.7.5
=====
* Make connection timeout period configurable.
* Don't panic on second call if server is off.
* Migrate gRPC interface to ES6 preview 3 version.

0.7.4
=====
* No change, only updated website link.

0.7.3
=====
* Expose subscription events so the user can be notified when a subscription has been confirmed or dropped by the server.
* Upgrade to uuid 0.8.* version.
* Fix memory leak in registry when dealing with subscriptions.
* Allow user to convert UUID to GUID when sending events.
* Fix persistent subscription regression when ack/nak.

0.7.2
=====
* Idiomatic streaming interface for subscriptions and batch reads (tcp API).
* Fix UUID/GUID serialization.

0.7.1
=====
* Ask Docs.rs to also build documentation for es6 module.

0.7.0
=====
* Expose ES 6 gRPC interface.

0.6.0
=====
* Move to tokio 0.2

0.5.1
=====
* Pinned `protobuf` to 2.8.1 version.

0.5.0
=====
* Support Rust 1.39
* Remove protobuf::Chars from public API.

0.4.5
=====
* Update persistent subscription default settings.
* Internal connection refactoring.
* Implement `iterate_over_batch`.

0.4.4
=====
* Remove debugging leftovers.
* Add `Pinned` system consumer strategy.

0.4.3
=====
* Fix reading a deleted stream event in $streams stream, causing a read command to abort.

0.4.2
=====
* Fix compiler warnings.
* Bump dependencies version.

0.4.1
=====
* Fix stream metadata and ACL JSON (de)serialization.

0.4.0
=====
* Implement cluster-mode connection.
* Internal refactoring.
* `start` and `start_with_runtime` are renamed `single_node_connection` and `single_node_connection_with_runtime`.

0.3.0
=====
* Migrate `iterate_over` from iterator to asynchronous stream.

0.2.4
=====
* Fix possible connection issues if Authentication or Identification processes take too long to complete.

0.2.3
=====
* Remove an unnecessary OS thread.
* Implement `ConnectionBuilder::start_with_runtime` to use an existing tokio runtime.
* Fix rare issue where the user sends a command before the connection is confirmed with the
  server, causing that operation to be sent only after a `operation timeout` time.
* No longer terminate the connection in case of identification timeout.

0.2.2
=====
* Implement stream streaming ($all included).

0.2.1
=====
* BUGFIX: Fix next event number for stream reads.

0.2.0
=====
* Simplify public eventstore module.
* Move to a typeful representation of `resolve_link_tos` setting.
* Implement connection state-machine graceful exit.
* Introduce new connection api.

0.1.3
=====
* Migrate to `uuid` 0.7.
* Move to tokio multithreaded runtime.
