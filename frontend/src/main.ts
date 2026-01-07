import './style.css'
import * as InitField from './init_field'

const myButton = document.getElementById("myButton") as HTMLButtonElement;

interface MessageInfo {
	message: string;
}

async function handleClick(event: MouseEvent) {
	try {
		const response = await fetch('api/title')

		console.log("here1");
		if (!response.ok) {
			throw new Error('HTTP error!')
		}
		console.log("here2");

		const data: MessageInfo = await response.json();
		console.log(data);

		const titleName = document.getElementById("titleName") as HTMLHeadingElement;
		titleName.textContent = data.message;

		InitField.createField()
	} catch (error) {
		console.error('Error fetching data', error);
		console.log("failed to get data");
	}
}

myButton.addEventListener('click', handleClick)
