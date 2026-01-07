interface TilesInfo {
	title: string,
	description: string,
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
	const firstRow = document.getElementById('field-first-row') as HTMLDivElement;
	const leftCol = document.getElementById('field-left-column') as HTMLDivElement;
	const rightCol = document.getElementById('field-right-column') as HTMLDivElement;
	const lastRow = document.getElementById('field-last-row') as HTMLDivElement;

	let tileNum: number = 0
	let [upRightAngle, downRightAngle, downLeftAngle] = generatePoints(tiles.tiles.length);
	for (let tile of tiles.tiles) {
		const tileDiv = document.createElement('div');
		tileDiv.className = 'field__tile';
		tileDiv.textContent = tile.title;

		if (tileNum <= upRightAngle) {
			firstRow.appendChild(tileDiv)
		} else if (tileNum < downRightAngle) {
			rightCol.appendChild(tileDiv)
		} else if (tileNum <= downLeftAngle) {
			lastRow.appendChild(tileDiv)
		} else {
			leftCol.appendChild(tileDiv)
		}

		tileNum += 1;
	}

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
