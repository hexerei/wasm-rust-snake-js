import init, { World } from "snake";

init().then(_ => {
  const world = World.new();
  const canvas = document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  console.log(world.width());
})