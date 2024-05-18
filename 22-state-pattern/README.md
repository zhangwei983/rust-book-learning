An example to show how to implement state pattern in Rust

## State Pattern

The state pattern is an object-oriented design pattern. The crux of the pattern is that we define a set of states a value can have internally. The states are represented by a set of state objects, and the value’s behavior changes based on its state. 

The state pattern is set to solve two main problems:

- An object should change its behavior when its internal state changes.
- State-specific behavior should be defined independently. That is, adding new states should not affect the behavior of existing states.

Please check the [Post example](./src/lib.rs) to see how to implement a state pattern with trait objects.

Baiscally, it shows that 
- Rust is capable of implementing the object-oriented state pattern to encapsulate the different kinds of behavior a post should have in each state.
- The logic related to the state pattern rules lives in the state objects rather than being scattered throughout `Post`.
- The methods on `Post` know nothing about the various behaviors.

 One downside of the state pattern is some states are coupled to each other because the states implement the transitions between states.

 Another downside is duplicating some logic.
 