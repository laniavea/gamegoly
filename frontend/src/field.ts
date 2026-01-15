interface TilesInfo {
	title: string,
	description: string,
	color: string,
}

interface Tiles {
	tiles: TilesInfo[];
}

async function fetchTiles(): Promise<Tiles> {
	const response = await fetch('api/get_tiles');
	if (!response.ok) {
		throw new Error(`While fetching get_tiles! Error code: ${response.status}`);
	}
	return response.json();
}

function validateTiles(tiles: Tiles) {
	if (tiles.tiles.length === 0) {
		throw new Error("Field do not contain any tiles");
	}
	if (tiles.tiles.length % 4 != 0) {
		throw new Error("Field can't be build because number of tiles not devidable by 4")
	}
}

function generatePoints(numOfTiles: number): [number, number, number] {
	const edgeNum = (numOfTiles - 4) / 4;

	const upRightAngle = 0 + edgeNum + 1;
	const downRightAngle = upRightAngle + edgeNum + 1;
	const downLeftAngle = downRightAngle + edgeNum + 1;

	return [upRightAngle, downRightAngle, downLeftAngle]
}

function createTiles(tiles: Tiles) {
	let tileNum: number = 0
	let [upRightAngle, downRightAngle, downLeftAngle] = generatePoints(tiles.tiles.length);

	let firstRowDivs: HTMLDivElement[] = [];
	let leftColDivs: HTMLDivElement[] = [];
	let rightColDivs: HTMLDivElement[] = [];
	let lastRowDivs: HTMLDivElement[] = [];

	for (let tile of tiles.tiles) {
		const tileDiv = document.createElement('div');
		tileDiv.style.backgroundColor = tile.color;
		tileDiv.textContent = tile.title;
		tileDiv.className = 'field__tile';

		if (tileNum <= upRightAngle) {
			if (tileNum === 0) {
				tileDiv.className += ' field__tile-top_corner-left';
			} else if (tileNum === upRightAngle) {
				tileDiv.className += ' field__tile-top_corner-right';
			} else {
				tileDiv.className += ' field__tile-top';
			}
			firstRowDivs.push(tileDiv);
		} else if (tileNum < downRightAngle) {
			tileDiv.className += ' field__tile-right';
			rightColDivs.push(tileDiv);
		} else if (tileNum <= downLeftAngle) {
			if (tileNum === downRightAngle) {
				tileDiv.className += ' field__tile-down_corner-right';
			} else if (tileNum === downLeftAngle) {
				tileDiv.className += ' field__tile-down_corner-left';
			} else {
				tileDiv.className += ' field__tile-down';
			}
			lastRowDivs.push(tileDiv);
		} else {
			tileDiv.className += ' field__tile-left';
			leftColDivs.push(tileDiv);
		}

		tileNum += 1;
	}

	reorderTiles(lastRowDivs);
	reorderTiles(leftColDivs);

	const firstRow = document.getElementById('field-first-row') as HTMLDivElement;
	const leftCol = document.getElementById('field-left-column') as HTMLDivElement;
	const rightCol = document.getElementById('field-right-column') as HTMLDivElement;
	const lastRow = document.getElementById('field-last-row') as HTMLDivElement;

	firstRow.replaceChildren(...firstRowDivs);
	leftCol.replaceChildren(...leftColDivs);
	rightCol.replaceChildren(...rightColDivs);
	lastRow.replaceChildren(...lastRowDivs);

	document.documentElement.style.setProperty("--tile-num", (upRightAngle + 1).toString());
}

function reorderTiles(generatedTiles: HTMLDivElement[]) {
	generatedTiles.reverse()
}

export async function createField() {
	try {
		const tiles: Tiles = await fetchTiles();
		validateTiles(tiles);
		createTiles(tiles);

	} catch (err) {
		console.error("Error in creating field: ", err);
	}
}
