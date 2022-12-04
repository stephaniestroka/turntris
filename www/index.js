import { Board, Cell, Direction } from "turntris";
import { memory } from "turntris/turntris_bg";

const CELL_SIZE = 25; // px
const BOARD_COLOR = "#DDDDDD";
const BOARD_STROKE_COLOR = "#333333";
const BLUE_COLOR = "#0099ff";
const PURPLE_COLOR = "#9933ff";
const ORANGE_COLOR = "#ff9933";
const GREEN_COLOR = "#00cc66";


const board = Board.new();

const canvas = document.getElementById("turntris-canvas");
canvas.height = (CELL_SIZE + 1) * board.length() + 1;
canvas.width = (CELL_SIZE + 1) * board.length() + 1;

const ctx = canvas.getContext('2d');

const sleep = (ms) => {
	return new Promise(resolve => setTimeout(resolve, ms));
}

const drawBoard = () => {
	ctx.beginPath();

	ctx.rect(0, 0, board.length() * (CELL_SIZE + 1), board.length() * (CELL_SIZE + 1));
	ctx.fillStyle = BOARD_COLOR;
	ctx.fill();
	ctx.lineWidth = 2;
	ctx.strokeStyle = BOARD_STROKE_COLOR;
	ctx.stroke();
}

const getIndex = (row, column) => {
	return row * board.length() + column;
};

const drawStones = () => {
	const cellsPtr = board.snapshot();
	const cells = new Uint8Array(memory.buffer, cellsPtr, board.length() * board.length());
	ctx.beginPath();
	for (let row = 0; row < board.length(); row++) {
		for (let col = 0; col < board.length(); col++) {
			const idx = getIndex(row, col);
			switch (cells[idx]) {
				case Cell.Free:
					ctx.fillStyle = BOARD_COLOR;
					break;
				case Cell.Blue:
					ctx.fillStyle = BLUE_COLOR;
					break;
				case Cell.Purple:
					ctx.fillStyle = PURPLE_COLOR;
					break;
				case Cell.Green:
					ctx.fillStyle = GREEN_COLOR;
					break;
				case Cell.Orange:
					ctx.fillStyle = ORANGE_COLOR;
					break;
			}
			ctx.fillRect(
				col * (CELL_SIZE + 1) + 1,
				row * (CELL_SIZE + 1) + 1,
				CELL_SIZE,
				CELL_SIZE
			);
		}
	}
	ctx.stroke();
}

drawBoard();

document.addEventListener("keyup", function (e) {
	e = e || window.event;
	console.log("key event: " + e.keyCode);
	// use e.keyCode
	if (e.keyCode == 37) {
		// move left
		board.move_stone(Direction.Left);
	} else if (e.keyCode == 39) {
		// move right 
		board.move_stone(Direction.Right);
	} else if (e.keyCode == 38) {
		// roate clockwise
		canvas.classList.add("rotate_clockwise");

		board.rotate_counter_clockwise();
		window.setTimeout(() => {
			canvas.classList.remove("rotate_clockwise");
		}, 500);
	} else if (e.keyCode == 40) {
		// roate counter-clockwise
		canvas.classList.add("rotate_counterclockwise");
		board.rotate_clockwise();
		window.setTimeout(() => {
			canvas.classList.remove("rotate_counterclockwise");
		}, 500);
	}
	drawStones();
});

const renderLoop = async () => {
	if (board.tick()) {
		drawStones();
		await sleep(500);
		requestAnimationFrame(renderLoop);
	} else {
		console.log('game over');
	}
};
requestAnimationFrame(renderLoop);
