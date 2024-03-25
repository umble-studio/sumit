const app = window.__TAURI__.app;
// const dialog = window.__TAURI__.dialog;
const http = window.__TAURI__.http;
const notification = window.__TAURI__.notification;
const fs = window.__TAURI__.fs;

window.fs_exists = async (path) => {
	const exists = await fs.exists(path, { baseDir: 6 });
	console.log("exists: " + exists);
	return exists;
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
