import './style.css'
import * as Field from './field'

window.onload = () => {
	try {
		Field.createField();
	} catch (error) {
		console.error(error)
	}
}
