import init, { SnakeGame } from "../snake-core/pkg/snake_core.js";

const GRID_SIZE = 40;
const TICK_INTERVAL_MS = 100;

async function main() {
  // 1) Initialize the WASM module (loads snake_core_bg.wasm behind the scenes)
  await init();

  // 2) Create a new SnakeGame, pointing to <canvas id="snakeCanvas">
  const game = new SnakeGame("snakeCanvas", GRID_SIZE);

  // 3) Hook up arrow keys to change direction
  window.addEventListener("keydown", (e) => {
    switch (e.key) {
      case "ArrowUp":
        game.change_direction(0, -1);
        break;
      case "ArrowDown":
        game.change_direction(0, 1);
        break;
      case "ArrowLeft":
        game.change_direction(-1, 0);
        break;
      case "ArrowRight":
        game.change_direction(1, 0);
        break;
      default:
        break;
    }
  });

  const scoreElement = document.getElementById("score-value");

  // 4) Game loop: on each animation frame, check if TICK_INTERVAL_MS has passed
  let lastTime = 0;
  function loop(timestamp) {
    if (!lastTime) lastTime = timestamp;
    const delta = timestamp - lastTime;
    if (delta >= TICK_INTERVAL_MS) {
      game.update();
      game.render();

      scoreElement.innerText = game.score;
      lastTime = timestamp;
    }
    if (!game.is_game_over) {
      window.requestAnimationFrame(loop);
    } else {
      console.log("Game Over! Final score:", game.score);
    }
  }

  // 5) Start the loop
  window.requestAnimationFrame(loop);
}

main().catch(console.error);
