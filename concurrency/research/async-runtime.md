# Intro
The project goal is to learn about Concurrency and Async.
I'm focusing primarily in Rust because I see it as a gap in my knowledge, the lack of understanding in Rust's fearless concurrency features.

The project idea came to me after watching about node internals and how nginx implement its event loop.
It seems like a good project to understand both concurrency and parallelism.
As I researched and learnt more about event loop and async models I realized there's far more I can do than what I originally expected.
So the project grew from implementing a callback based event loop to implementing an asynchronous runtime in Rust using Futures, thread pools and so on.
From this project I expect to learn about fearless concurrency, thread spawning, Send + Sync, async and Futures.

# Asynchronous runtimes
What became cleara to me after I started learning about this is that there are multiple ways to handle asynchronous code.
Although all the techniques I've so far ultimately boil down to callbacks, there have been some interesting and useful abstractions created to help mitigate some of the mental gymnastics that comes with callback usage.

I will focus on a simple event loop implemention (a la nginx) and on the new and more sophisticated model based in promises and coroutines.

With that said though, it's important to outline what an asynchronous runtime does, is and what it solves.

## Overview
TLDR there are many situations in computing where some form of concurrency is needed.
The primary tool used for concurrency is threads.
Threads are great as they offer a high level abstraction where the programmer is able to think about their code lineraly without havingt to worry about multitasking.
This is possible due to the preemptive nature of threads, where the runtime will use some sort of scheduler algorithm to decide which threads to run.
With that said, threads come at a cost and are not useful in every situation.

Most notably, threads aren't suitable for IO bond processes such as a web server.
IO sys calls are often blocking (eg. read(), write()), which means the thread will freeze.
If a webserver is to handle thousands of requests / sec with an acceptable response time, creating a thread for each new connection is unfeasible as the cost of switching between threads piles up.
Async is especially good for this situation because a single thread can handle multiple connections.
At its heart, the async runtime will use non-blocking syscalls to handle IO and will do some sort of polling / signal handling to progress through a connection.

An asynchronous runtime are tools that aid building asynchronous applications using the aforementioned techinques.


## Event loop
The event loop is at its core a simple and effective model  for an async runtime.

One of the things I had to understand was what did the event loop respond to.

Again, nginx docs were awesome in this.

Since the idea of an event loop is to allow blocking tasks to be done in a non-blocking manner, it makes sense that event loops provide a way to perform said tasks in an async fashion.

The two big APIs an event loop should provide is: non-blocking timer and non-blocking IO.
Non blocking IO allows monitoring multiple file descriptors which in the linux world mean basically anything (tty, files, sockets, pipes, ...).
Timers allow for task scheduling at an appropriate time, or to delegate an event.

There are multiple non-blocking io options in linux.
i read about some of the alternatives such as signals, epoll, non-blocking bit in open and so on.
The linux programming interface book was great for that.

For timers there are two solutions, one in user space and another at OS level.
Nginx uses an user space approach where timers are only approximate, there's no guarantee of when a timed out event will be handled, only that it will NOT be handled before the timer expires.
Nginx uses a binary tree for that, where the leaves are timer expiration instant and at the start of each event loop iteration all timers will be handled.
An OS approch makes use of signals or the many apis in Linux for that, again the Linux programming interface book is a great reference.

## Promise based runties + coroutines
As I said, event loops are simple and effective but they can become complex.
Things get messy when there are too many nested callbacks and we have a callback hell.
The code can also be difficult to understand in some situations and it's difficult to reason about it.

A different model for async runtimes based on promises became popular in a few languages.
it's been implemented in C#, Java, Rust and a bunch more that I don't rly know about.

The idea is interesting and somewhat similar to threads: allow the programmer to write code as though it will be executed linearly.
That's it.

The way in which that works is far from simple though.
The TLDR is that programs can call asyncrhonous functions which will return a Promise for the eventual return value.
The calling code can `await` said Promise to extract the result.
With the extracted result the program can operate over it and carry on with its execution.
Multiple async calls can be done in the same method.

Under the hood, what happens is that the `await` call essentially sets up a coroutine + callback mechanism.
awaiting on an async result registers the current function at the current line as a callback to the promise.
Once the promise is fullfilled, the current function will be executed from that point onwards.
The mechanism of "pausing" and "contining" a function is essentially what a coroutine is, except that the final user does not have as much granularity with it.
Under the hood however, the compiler converts the async function in something analogous to a courintine (puase / resume) through a state machine.

This was illustrated in the Rust talk, c# documentations and c# article about async

# Async runtime implementations

## Rust await

Rust provides language support for a task based asynchronous runtime.

The main components responsible for this implementation are:
- `.await`
- `Future` trait
- `async` function keyword
- Waker

In essence, Rust's implementation of its async runtime is very similar to C#'s, except there are a few important differences.

A Rust Future is lazy and its execution is only initiated once the Future's `.poll` method is invoked.
That is in __sharp__ :^) constrast with C#, where Tasks are eager until an await call is made, at which point the flow control returns to the caller.

Futures are passed to Executors, which are responsible for orchestrating the runtime and polling the Futures (either directly or through a thread poll as C# does).

Once a Future is ready to return its value or to progress through, it needs to signal the executor somehow.
The Waker is the mechanism that enables a Future to resume its exucution.

Wakers are passed to a Future whenever `.poll` is called, Futures are responbile for storing a refenrece or copy to a waker and calling it whenever it's ready to resume its execution.

Supposed a fd + epoll scenario in Linux and a Future watching the fd.
Once the thread blocked at the epoll fd resumes its execution, it must signal the Future that its fd is ready, the future in turn will use the waker to signal the executor that its ready to make progress.

Internally, Rust's mechanism resembles c# in a lot of ways but it differs in that it's lazy by default and the complete runtime implementation is not built into the language.
It's a zero cost abstraction.

C# tasks is a compiler generate construct which compiles to state machine.
Any async function is a Task.

I accidently stumbled upon a rust talk on how futures are implemented and it was very interesting, couple things stand out:
- It validated the view I had about my event loop architecture, the runtime was tied to the task q logic.
- Reminded me of the Continuation Monad from Haskell, async coroutines and so on are tighly related, it seems that all those things are a type of continuation
- The Rust model for async is very intricate in that it deal with tasks, not callbacks, and tasks are passed to executors. The event loop is broken into multiple pieces. (It seems like C# has a similar model).

[Tokio async in depth](https://tokio.rs/tokio/tutorial/async)
[Future trait](https://rust-lang.github.io/async-book/01_execution/02_future.html)


## Python AsyncIO

Python's AsyncIO is a full featured asynchronous runtime.

Reading the docs, what stood out to me is that there are A LOT of features, far more than I saw from nginx.
It makes sense since this is supposed to be a generic runtime, and not an application specific one.
With that said, it seems asyncIO has features that don't belong in an event loop, which again, makes sense becuase this is an async runtime i guess.

For instance, coroutines can put themselves to sleep, tasks can wait for each other, subprocesses can be created asyncronously.
This was surprising to me because from my understanding an event loop just takes callbacks and runs them, much like how it's done in Node.

The main entities of the AsyncIO runtime are:
- Coroutine
- Task
- Future
- Event Loop

Coroutine is a  routine with multiple entry points.

Task is a high level api which can be used to start coroutines, other than starting coroutines they can also put coroutines to sleep or cancel them entirely.
These features from tasks remind me a lot of job scheduling.
I thought the event loop was a dumb thing that did not control the functions or routines that were being executed.
At first I thought about adding a timeout to when a task was taking too long but it seemed outside of the scope of the event loop.

Future is the prevalent abstraction for a value that will be available in the future.
Reminds me of the Maybe and IO monad together.

Event loop is the runtime itself.
Through the event loop one can do a lot of things such as handle signals, set options, create subprocess, register callbacks, create tasks and futures.
The interesting thing here is that the event loop is a single thing that handles all of the above.
Maybe that's because it's the public API.
It kind of aligns with how I was doing things originally but the point is that what python calls the event loop seems to be the runtime itself.

[Coroutine and tasks](https://docs.python.org/3/library/asyncio-task.html)
[Event loop](https://docs.python.org/3/library/asyncio-eventloop.html)
[Futures](https://docs.python.org/3/library/asyncio-future.html)
[PEP 492 -- Coroutines with async and await syntax](https://www.python.org/dev/peps/pep-0492/)


## Coroutines
Studying about Python, the terminology of a coroutine was introduced.
I heard about coroutines before while studying the haskell continuation monad and again in Lua, so I decided to study about it.

Coroutines are basically functions that can be stopped and resumed mid execution.
It's very similar to how a thread behaves, in that its context is preserved but it differs in that it's cooperative.

Both threads and coroutines allow for multitasking.
Blassic threads make use of preemptive multitasking where there is a runtime responsible for stopping and resuming the threads through a scheduling algorithm.
The key is that the thread run a simple function that may be frozen.

Cooperative multitasking differs in that each function is responsible for yielding control back to the caller.
That means that coroutines must be implemented such that they play nice with each other, avoiding consuming too much CPU time at once and being mindfull of when a call might block.

There are different types of coroutines, assymentrc and symetric.
The Lua author explains their difference in an article.
The TLDR is that symmetric coroutines make use of a single call to yield / resume.
Assymetric have 2 different calls for that.
It's argueed that asymmetric coroutines are simpler because they resemble functions more closely.
Flow of control gets complicated with symmetric coroutines.

Note that it's possible to implement symmetric coroutines on top of an asymmetric coroutine.

It's interesting to note that, coroutines can be implemented with the continuation monad in haskell.
Continuations seem like a very powerful concept.

My main take away here is coroutines are related to async programming, in that context execution is swapped between them but not necessarily.
Coroutines go above andd beyond the vanilla callback-based event loop implementation.
They seems like a powerful tool but I don't think I will implement them in this current project, although they are yet another tool for multitasking.

[Coroutine tutorial](http://lua-users.org/wiki/CoroutinesTutorial)
[Coroutines in Lua](http://www.lua.org/doc/jucs04.pdf)


## C#

C# seems to have a similar model to that of Rust regarding async.
The meat and bones of it all is the trinity: `async`, `await` and `Task`.

Taks is analogous to Future, it's an object that will return a value when awaited.
`await` is a keyword that sets a point in the async method where the control flow depends on the completion of the async task.
`async` is used as a prefix in method declaration to signal that the method will be handled asynchronously.

The article was insightful but it only scratched the surface.
The notorious bit of information is that when an async method is called, it's executed until the first `await` keyword.
At that point, the method yields control back to the caller and the compiler does its magic to create the `Task` object.
The `Task` object received by the caller is essentially a handler for the asyncronous task going.
Once the async task finishes, its result will be stored inside the Task object.
Extracting a value from a `Task` is done by calling `await` on it.
eg: `let result = await methodThatReturnsTask()`

This is *very* similar to the notion of continuations in my pupper haskell.

The initial article gives an overview of this API from an user POV, I want to understand the meat of it.

The async in depth article was 3% interesting BUT it gave me a direction, the Task based approach is the same as the Future and Pormise one.

What was interesting is that the Future / Promise / Task runtime is, also, more generic than simply non-blocking calls and promises.

The task based model in C# can spin up a new thread, or use one from a thread poll to initiate a task.
It's not necessary that the entire line of execution happen in a single thread.
They key of it is, don't block a thread.

This also made me realize that I should structure this research a bit differently.
I was reading about Async models for different languages, now I need to read about futures and promises and how they are implemented.
With that said, it's clear to me that they are still implemented using a callback of sorts, it's just a much more generic and abstract API

> On the C# side of things, the compiler transforms your code into a state machine that keeps track of things like yielding execution when an await is reached and resuming execution when a background job has finished.
For the theoretically inclined, this is an implementation of the Promise Model of asynchrony.

https://docs.microsoft.com/en-us/dotnet/standard/asynchronous-programming-patterns/
https://docs.microsoft.com/en-us/dotnet/standard/parallel-programming/tpl-and-traditional-async-programming
[Task async programming model](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/concepts/async/task-asynchronous-programming-model)
[Async in Depth](https://docs.microsoft.com/en-us/dotnet/standard/async-in-depth)


# Futures and Promises

Futures are a wrapper over a value that will be available.
They provide a read-only view of said value.

Promises are used to compute a future.

Future specify values that will exist, Promise specify how that value will be crated.

The wikipedia article gives an overview of the idea but the interesting thing to note here is that it seems the notion of Promise and Future isn't tied to any implementation scheme.
The way in which the promise future is resolved is very much up to the language, being possible to have both blocking or non-blocking implementations.

This leads me to think about C# models in which the Future is non-blocking and it uses an internal callback mechanism to handle the control flow for an async function.
That is what I am interested about, how the control flow is achieved, not much about how the future is modeled.

[Futures and Promises](https://en.wikipedia.org/wiki/Futures_and_promises)


# Read
https://docs.microsoft.com/en-us/dotnet/standard/parallel-programming/task-based-asynchronous-programming
https://docs.oracle.com/javase/10/docs/api/java/util/concurrent/package-summary.html
https://docs.microsoft.com/en-us/dotnet/standard/asynchronous-programming-patterns/task-based-asynchronous-pattern-tap
https://docs.microsoft.com/en-us/dotnet/csharp/async

So next steps is to understand how the fuck the task based async model works, or at least how everything interact.
With that said, it's far more involved than i previously thought


# Callback approach

My first intution was to get started in this project based on my understanding of an event loop, which involved callbacks.
That's how it is done in Nginx and that's how it's done in Java script / browsers.

I put some work into it and I got to learn about some Rust stuff, like the time std lib, more about Box, Rc, Arc; trait objects again.
Overall, I am interested in more type theory stuff, that's something I want to learn about eventually.

During that journey it made sense to me to mimick a model used by nginx, one in which an event stores the callback data.
After reading about an alternative in which the callback function is a closure that seemed to make more sense because the callback does not care what data the callback will receive, that is solemly a concern of the user application.
the drawback is that it becomes harder to log and give context when a callback fails without the payload.
the upside is that it simplifies type system stuff quite a bit.

The one thing I started questioning about my architecture is that the event loop was tied with the runtime.
I started questioning that design decision, because the event loop is simply the task queue, it handles the events and each event produces a result.
it shouldn't deal with runtime stuffs, that's a different entity (retry logic, sleeping when task q is empty, signal response and so on).


# Conclusions

Async based runtimes are an alternative approach to concurrency.
There are several approaches to concurrency such as event loops, event based, threads, async runtimes.
As always, they all have pros and cons.

Given that async runtimes are more adequate to IO bound workload with several events / files being monitored, this leaves us with some choices.

The main advantage of the promise based runtime is that the complexities inherent from callbacks are simplified through a nicer syntax.
The compiler and the async runtime are responsible for managing the callbacks and event handling.
It's a higher level approach to an old problem that seems __promising__ :^).

Promise based runtimes such as pythons asyncio, c# tap, rust async are a declarative approach to managing asynchronous workloads which attempts to make async as familiar as synchronous.
as usual, there are concurrency synchronization challenges involved.

Are async runtimes less prone to deadlocks / race conditions?


# Refs

[nginx event loop](https://nginx.org/en/docs/dev/development_guide.html#event_loop)
[talk on node event loop](https://www.youtube.com/watch?v=8aGhZQkoFbQ)
[rust future/async](https://www.youtube.com/watch?v=NNwK5ZPAJCk)
[html event loop spec](https://html.spec.whatwg.org/multipage/webappapis.html#event-loop)
[python async io](https://docs.python.org/3/library/asyncio.html)
