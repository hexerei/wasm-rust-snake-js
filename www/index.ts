import init, { World, Direction, GameState } from "snake";
import { rnd } from "./utils/rnd";

init().then(wasm => {

  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const SNAKE_DIR = Direction.Right;
  const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx, SNAKE_DIR);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById("game-control-btn");
  gameControlBtn.addEventListener("click", _ => {
    const gameState = world.game_state();
    if (gameState === undefined) {
      gameControlBtn.textContent = "Playing...";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  const gameStateLabel = document.getElementById("game-state");
  const gamePointsLabel = document.getElementById("game-points");


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

  function drawReward() {
    const idx = world.get_reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);
    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {

    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    )

    ctx.beginPath();
    snakeCells
      .filter((c, i) => !(i > 0 && c === snakeCells[0]))
      .forEach( (cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);
      ctx.fillStyle = (i < 1) ? "#7878db" : "#333333";
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    ctx.stroke();
  }

  function drawGameState() {
    gameStateLabel.textContent = world.game_state_text();
    gamePointsLabel.textContent = world.points().toString();
  }

  function paint() {
    drawWorld();
    drawSnake();  
    drawReward();
    drawGameState();
  }

  function play() {
    const state = world.game_state();
    if (state == GameState.Won || state == GameState.Lost) {
      gameControlBtn.textContent = "Play Again";
    } else {
      const fps = 3;
      setTimeout(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        world.step();
        paint();
        // takes a callback to be invoked before next repaint
        requestAnimationFrame(play);
      }, 1000 / fps);  
    }
  }

  paint();

})