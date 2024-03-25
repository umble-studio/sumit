const app = window.__TAURI__.app;
// const dialog = window.__TAURI__.dialog;
const http = window.__TAURI__.http;
const notification = window.__TAURI__.notification;
const fs = window.__TAURI__.fs;

// import { readFileText } from '@tauri-apps/plugin-fs';

window.fs_exists = async (path) => {
	const exists = await fs.exists(path, { baseDir: 6 });
	console.log("exists: " + exists);
	return exists;
}

window.fs_readTextFile = async (path) => {
	const text = await fs.readTextFile(path, { baseDir: 6 });
	console.log("text: " + text);
	return text;
}

window.fs_readFile = async (path) => {
	const buffer = await fs.readFile(path, { baseDir: 6 });
	console.log("buffer: " + buffer);
	return buffer;
}

window.getTauriVersion = async () => {
	const tauriVersion = await app.getTauriVersion();
    console.log("version: " + tauriVersion);
	return tauriVersion;
};

window.sendNotification = async (message) => {
	let permissionGranted = await notification.isPermissionGranted();
	if (!permissionGranted) {
		const permission = await requestPermission();
		permissionGranted = permission === "granted";
	}
	if (permissionGranted) {
		notification.sendNotification(message);
	}
};
