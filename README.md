# ALOUETTE ONE

## Welcome!

To my humble repo!

This repository contains a very much work-in-progress game that I am making "from scratch" in an effort to learn Rust, and to get back into game development.

"from scratch" meaning:

- I'm using low-level graphics APIs - so far only DirectX
- Writing as much of the lower-level details pertaining to the operating system as possible - for example creation of the window, the DirectX context, etc.
- Decoding and playing sound using low-level audio APIs.
- Writing my own math library geared towards game development, including a library for linear algebra useful for 3D rendering and game programing
- etc... I think you get the idea ;)

## The Code

If you've come here trying to deduce best-practice design patterns and solutions for Rust and game engine programming in general, you should turn around immediately. Turn around, and don't look back. Please.

I am in the process of learning both Rust and much low-level programming all at the same point. This project is a result of that. It's a living, breathing organization of code that will slowly be refined over time, but is at this point very rough. By the mere fact that I haven't attempted proper completion of any project of this sort before.

In order to be productive I live by a very simple principle: Shoot first, ask questions later. Or said in another way, **Prototype rapidly first, refactor later**.

This comes from what Steve McConnell refers to as the **wicked problem** in *Code Complete 2*. The paradox that a problem can only be clearly defined by solving it, or solving part of it, first.

That is, you have to solve a problem once, in order to even understand it, so that you can solve it better a second time around. Said in another way: **It's a waste of time to attempt a clean design / clean code before you even know what it is you're trying to solve.**

Being faced with a new problem, such as opening a window using the Windows API, creating a DirectX context, or whatever else concrete task you can come up with, is essentially a giant set of unknown variables that you have to solve for. If you don't know the unknowns, how do you ever expect to create a clean design that will encompass them? The fastest way to discover as many unknowns as possible - in order to reach a clean design *later* - is to get going, and produce code that aims at solving it.
