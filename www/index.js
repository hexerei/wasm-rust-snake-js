import init, { World } from "snake";

init().then(_ => {
  const world = World.new();
  console.log(world.width());
})