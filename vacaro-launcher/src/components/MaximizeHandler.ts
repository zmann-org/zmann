import { appWindow } from '@tauri-apps/api/window';

function GetisMaximized() {
	appWindow.isMaximized().then((value) => {
		console.log(value);
		return value;
	});
}

export default GetisMaximized;
