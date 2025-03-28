
# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
```sh
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
```

    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.



## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

* We use utilize RwLock<> than Mutex<> because Vec of Notifications are read more frequently but written less frequently. RwLock<> allows many reads simultaneously but maintains exclusive write access, which will be better performing than using Mutex<>, as it would lock out every thread from taking any action, reducing performance and concurrency.

* Rust doesn't allow mutating static variables directly (without using mechanisms like `lazy_static`) since its basic priority is memory safety and thread safety guarantees. Unlike Java, where you are free to modify static variables through static methods, Rust has more stringent rules for a variety of important reasons:

1. **Thread Safety**: Static variables in Rust have a 'static lifetime and can be accessed from any thread at any time. Mutation without synchronization would result in data races when used in multithreaded environments.

2. **Ownership System**: Rust's ownership system, which prevents memory safety issues, does not find an owner for static variables since they exist throughout the life of the program.

#### Reflection Subscriber-2

- **Exploring beyond the tutorial steps**

    Yes, I explored into the `src/lib.rs` file that contains the core implementation of the notification system. Out of this experience, I gained knowledge of many important concepts:

    - The application of the Observer pattern through the `Publisher` trait and `Subscriber` trait, showing Rust's trait system enabling clean interface definitions
    - Rust's support for concurrency via atomic reference counting (`Arc`) and read-write locks (`RwLock`)
    - Use of `DashMap` as a lockless concurrent hash map implementation supporting thread-safe access without locking on each operation
    - Rust async/await syntax implementation for handling WebSocket connections asynchronously
    - Implementation of serde-based serialization/deserialization for sending notifications between services

    This study taught me why the notification system architecture supports decoupled publisher-subscriber communication and how Rust's safety features extend to concurrency.

- **Scaling with Observer pattern**

    Observer pattern renders it extremely simple to introduce new subscribers in the system because:

    - New subscribers need to implement the `Subscriber` trait without modifying the publisher code
    - The registration process allows dynamic addition and removal of subscribers at runtime
    - Subscribers are able to process notifications independently according to their respective individual needs

    When spawning multiple instances of the Receiver app, I found it simple to associate each instance with the notification system because the observer pattern handles the association and notification dispatching for you.

    Scaling with multiple instances of the Main app (publisher) however brings some additional issues:

    - Each Main app instance would need some way of remembering which notifications had already been sent in order to prevent duplicates
    - There would be a need for a central registry or message broker to notify all subscribers from all publisher instances
    - The system would need additional logic to handle failure of publisher instances or network partitions

-  **Postman Collection**

    I have included tests in the Postman collection to confirm my endpoints, which include listening to various product kinds and having numerous subscriptions. This has helped to guarantee that the endpoints function as planned and provide accurate data. The system's general stability is increased by writing tests and increasing documentation, which also aid in handling edge cases, validating both happy and unpleasant pathways, and checking code coverage.
