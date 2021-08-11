# OSIntegrationServer

Provides operating system specific functionality to NodeJS applications utilising communication with a compiled binary.

## Architecture

This application will be designed with 1 Major thread, and seperate threads which allow the bolting on of additional features:

![Design](https://user-images.githubusercontent.com/7938900/124355069-3b238f00-dc07-11eb-9da1-7efa1c5ad7c9.png)


The `OSIntegrationClient` TypeScript environment will act as an API end point for NodeJS applications. This client's endpoints will send and receive messages to/from the server over IO (using `MessagePack`/`JSON` encoding). The client will also be responsible for generating a unique ID for the messages it sends.

The server itself will:

* manage all IO from 1 thread, store all messages received in a linked-list(?) and give each message an internal ID.
* seperate threads providing features will loop over the linked-list, take in requests, process them and schedule them for garbage collection.

