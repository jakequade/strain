# Learnings

The following are things I've learned about how Space Menace was built, and the system as a whole.

## Everything fluid is a System, Everything needing a state is a component.

- For example, an animation that animates the location of a `Person` component shouldn't happen in a `PersonSystem`, or even a `WalkingSystem`. There should instead be an `AnimationSystem` that modifies the `Transform`s themselves.

## Prefabs are your friend

- Especially for animations!

## Create multiple states for game states

## Pipeline: how this project works

