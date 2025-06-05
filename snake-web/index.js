import init, { SnakeGame } from "../pkg/snake_core.js";

const GRID_SIZE = 20;
const TICK_INTERVAL_MS = 150;

async function main() {
  // 1) Load the WASM and wait until it’s ready
  await init();

  // 2) Create a new SnakeGame, pointing at <canvas id="snakeCanvas">
  const game = new SnakeGame("snakeCanvas", GRID_SIZE);

  // 3) Hook up arrow keys
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
    }
  });

  // 4) Grab the <span> that shows the score
  const scoreElement = document.getElementById("score-value");
  if (!scoreElement) {
    console.error("No element with id='score-value' found!");
    return;
  }

  // 5) Game loop
  let lastTime = 0;
  function loop(timestamp) {
    if (!lastTime) lastTime = timestamp;
    const delta = timestamp - lastTime;

    if (delta >= TICK_INTERVAL_MS) {
      // a) Advance game state
      game.update();
      // b) Redraw canvas
      game.render();
      // c) ***Update HTML score*** (use property, not method)
      scoreElement.innerText = game.score;

      lastTime = timestamp;
    }

    if (!game.is_game_over) {
      window.requestAnimationFrame(loop);
    } else {
      console.log("Game Over! Final score:", game.score);
    }
  }

  window.requestAnimationFrame(loop);
}

main().catch(console.error);
