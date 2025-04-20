# Entity Component System

this engine uses an approach to managing
game state often called an "entity component
system" or ECS.

in this system, there is always one (and only
one) active `World`. this `World` is then 
responsible for managing so called `GameObjects`, 
which are essentially just _things_ in your game
world.

each `GameObject` can then have multiple
`Components` attached to it, which determine how
the `GameObject` behaves.

## terminology

to follow rust's overall aesthetic, this ECS uses
the following data-types.

- `Scene`
- `Entity`
- `Component`

there are also some rust-ecs specific things

- `System`

in contrast to many ECSs in c-like languages,
in rust, it is common practice to not have your 
`Components` handle the actual logic, but rather
have `Systems` do the work. These `Systems` are
essentially functions that get called with every
`Entity` which has a `Component` of a certain
type attached to it.

this effectively splits the work between 
`Components` and these `Systems`, making a
`Component` purely a way of storing information
and marking game objects.
