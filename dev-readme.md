# How I developed this game

## Steps

1. Create the game state, in `Strain.rs`.
2. Create the spritesheet and its loader.
  1. Create a loader resource on the world: `let loader = world.read_resource::<Loader>()`
  2. Create a texture storage on the world: `let textures = world.read_resource::<AssetStorage<Texture>>();`
