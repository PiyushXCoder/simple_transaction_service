# Design Documentation

I am not sure how to make this documentation and I am typing it with hand. So please forgive me if I don't know the actual format of doing it.

## Atomic Transactions

I am utilizing mysql's transaction and lock to achieve true atomic transactions. But here is all the thoughts givento it!

So when I got the problem statement. I stumble the word atomic. As a computer science student it have a lots of thinking to be given. First of I had read that the SQL db follows ACID, where `A` is for Atomic.

But then how do I know if it really is atomic.

So what is atomic? The question is very easy to answer. If a set of of operation can roll back without affecting the actual system in case of failure at any step, we can call it an atomic operation. Easy?

NO!

The main problem is concurrency. First of all I though maybe the question want me to develop an atomic design using those popular design patters. Have state refered as momento and storing them and in case of failure rolling back!

But after struggling bit! I understood that not easy to do.

Let me explain!

So to make any system atomic we need to do following.

1. Begin a transaction. That is like making a new branch in git.

2. Lock all the resource one by one (Why? So no one makes a bad decision based on race), as an analogy imagine if you are working on a file in you branch and someone really edit it in main. It will make merge conflict

3. Modify everything

4. and commit all

5. release locks

If anything is worng till step 3 then noting bad can happen

but what if one of the commit fails?

Thats a problem. now you can't rollback the commited data.

I was trying to find solution to it. But then I didn't find anything reliable. But I got to know that people depend on database for such situation.

If i have to modify only one database then I can use mysql transactions with locks.

## Structure of project

```
 .
├──  API-Documentation.md --> API Documentation
├──  Cargo.lock
├──  Cargo.toml
├──  DESIGN.md
├──  docker-compose.yml
├──  Dockerfile
├──  migrations --> It contains migrations
│ ├──  20250919063118_api_keys.down.sql
├── 󱧼 src
│ ├──  actor_webhook_service_impl --> This contains sqlx implementation for `webhook_service`
│ ├──  bin --> contains binaries. `server` is one which have server
│ ├──  controller --> It contains controllers
│ ├──  core --> This contains actual logics (Independent of `controller`)
│ ├──  db --> Interface for database(independent of `sqlx`)
│ ├──  errors.rs --> errors :)
│ ├──  lib.rs --> exports everything to main
│ ├──  messages --> Massages to be shared through `controller` to `core`
│ ├──  router.rs --> Starting point for the server
│ ├──  sqlx_db_impl --> This contains sqlx implementation for `db`
│ ├──  validator.rs --> Validates keys
│ └──  webhook_service --> Interface for webhook(Independent of `actor_webhook_service_impl`)
├──  target
└──  webhook-server.js
```

## I have tried to use lessons from clean architecuture

![](Diagram.webp)

So there is over all internal architecure I have tried my best to design it in a way so we can plug better infrastructure in future.

For example look at sqlx, we may want something else? replace it! Or the webhook actor. We should create completely seperate service with a good queue like rabbitMQ.

I have used clean architecture. Few thing to note are that Infrastructure is kept seperate from actual business logic as we might change infrastructure later. Tesing of business logic can be done using demo infrastructure as we have interface for infrastructure. Even the business logic is tried to keep loosly coupled with Actix_web, I am not sure when it might not work and we re build controllers.

For Webhook I have utilized actors provided my actix. I did spin a actor which waits for poll lazily and if there is poll from broker. I does pull latest messages in queue and sends all.

## Things I could have had improved

I should have used AI more to do the repetative tasks. I was tring to structure very thing and reason round. My goal was to give lots of space to upgrade.

I should have had used workspace as that could have help me make a better seperation between modules which are independent such as `core`, `actor_webhook_service_impl` and`sqlx_db_impl`.

Also i would have got chance to combine common things
