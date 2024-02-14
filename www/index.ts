import init, { World, Direction } from "snake";

init().then(wasm => {

  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const SNAKE_DIR = Direction.Right;
  const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx, SNAKE_DIR);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  const snakeCellPtr = world.snake_cells();
  const snakeLen = world.snake_length();
  const snakeCells = new Uint32Array(
    wasm.memory.buffer,
    snakeCellPtr,
    snakeLen
  )

  document.addEventListener("keydown", key => {
    switch(key.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    // vertical grid lines
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
    }

    // horizontal grid lines
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
    }

    ctx.stroke();
  }

  function drawSnake() {

    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    )

    ctx.beginPath();
    snakeCells.forEach( (cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);
      ctx.fillStyle = i < 1 ? "#7878db" : "#000000";
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });
    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();  
  }

  function update() {
    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      // takes a callback to be invoked before next repaint
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  update();



})