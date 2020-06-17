# How this (and strain) work

## Background

I was keen on making a 2D platformer in rust. Unfortunately, the amethyst book didn't go into depth on how platformers would work. I decided I would carbon-copy space menace (part of the amethyst showcase) and, in the process, document how the whole thing works. That is, in the absence of the author having done it.

## General game architecture

The code is divided up into a number of groups: components, entities, resources, states and systems. Here's how they each work.

### Components

Components represent the amethyst `Component`s that are attached to `Entity`s. Their job is to solely define the properties and construction of their individual items (e.g. the struct `Dude` is made up of a `state: DudeState` and a `health: i32`). The components in this example include:

- `Animation`: A single animation from a loaded prefab. It also knows about the entire spritesheet.
- `Collider`: A set of boundaries that is applied to objects. Used for interaction with enemies and/or floors/walls.
- `Direction`: The direction an animation is happening in, and an enum of all possible directions. Used to determine how animations will progress from state to state.
- `Dude`: In space menace this is `Marine`, but it's essentially the protagonist. There is an enum of states the protagonist can be in (walking, idle, shooting etc), and the implementation designates some hardcoded values like the max walking/jumping speeds it can reach.
- `Motion`: Used to calculate the math behind a transformation or component movement. It applies velocities, and reasons about things like how an entity acts in mid-air, or with no colliders beneath them.
- `Prefabs`: This is responsible for taking your prefab ron and converting its data into a valid struct. It's also responsible for initialising all animations within the `World` by way of initialising any animated sprites with a state. In this example, that state is `Idle`.
- `Subject`: This is `NullStorage` type that is simply used to attach one component to another. In this game, it attaches the `Camera` entity to the `Dude`, so that it follows them as they move through the stage.

The simplest definition for components is that they define the basic structures of your game. Any work that relates to these components happens either in systems or entities - not the components themselves.

### Entities

Entities (in this context) have a few jobs:

- Where they relate to a component, they define how the component's state is altered or instanced.
- They have a function dedicated to loading components into the game's `World`.
- Where they don't load components, they usually add other elements into the `World`. One example is instancing the camera.

The entities files in this example are:

- `camera` / `camera_subject`: Instances the camera that the player will be using to view the game.
- `dude`: Instances the protagonist.

### Resources

Resources are `.rs` files that relate to loading non-code into your program. In this example, there's two:

- `assets.rs`: loads the prefabs and animations into the `World`.
- `context.rs`: provides hardcoded values such as window size and map dimensions. Ideally, this would be loaded from a prefab so it didn't require recompiling.

### States

States are game states. When the game is instanced, it's instanced with a `Loading` state. When the assets are loaded, they progress through states depending on game logic and input.

### Systems

Systems collectively determine both what is happening in the game right now, and how that will progress. They are responsible for dictating game progression on every iteration of the game loop, and then make changes that they will then evaluate next game loop. The examples in this project are:

- `animation`: starts and stops animations within the game. A simple example is going from an idle animation to a walking one when the user inputs a direction.
- `input`: receives inputs from the user and changes component states accordingly.
- `motion`: applies velocity if an input is present, but also determines course-of-action if objects collide (e.g. a player getting knocked backed if colliding with an enemy, or not progressing if colliding with a wall).
- `physics`: similar to motion, but relates to how components should act against things like "gravity" (e.g. falling should involve acceleration, up to a certain ceiling).
- `transformation`: applies the information from other systems to the `Transform` instances they relate to. For example, when a user inputs a direction:
  - the `animation` system will start the walking animation
  - the `motion` system will apply an acceleration value
  - the `transformation` system will join all these events and apply the relevant translations against the `Transform` instances tied to those systems.

## What happens when you run `cargo run`

TODO: Run through how all the above is initialised, starting at a top-to-bottom of `main.rs` and `strain.rs`

## How the Systems connect to each other