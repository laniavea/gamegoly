import './style.css'
import * as Field from './field'

const myButton = document.getElementById("myButton") as HTMLButtonElement;

interface MessageInfo {
	message: string;
}

async function handleClick(event: MouseEvent) {
	try {
		const response = await fetch('api/title')

		if (!response.ok) {
			throw new Error('HTTP error!')
		}
		const data: MessageInfo = await response.json();
		console.log(data);

		const titleName = document.getElementById("titleName") as HTMLHeadingElement;
		titleName.textContent = data.message;

		Field.createField()
	} catch (error) {
		console.error('Error fetching data', error);
		console.log("failed to get data");
	}
}

myButton.addEventListener('click', handleClick)
