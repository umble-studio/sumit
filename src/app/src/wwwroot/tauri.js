const app = window.__TAURI__.app;
// const dialog = window.__TAURI__.dialog;
const http = window.__TAURI__.http;
const notification = window.__TAURI__.notification;
const fs = window.__TAURI__.fs;
const invoke = window.__TAURI__.core.invoke;
const event = window.__TAURI__.event;

// import { readFileText } from '@tauri-apps/plugin-fs';

window.onload = async () => {
    // await event.emit("click", {
    //     name: "sumit",
    // })
    
    await event.listen("ON_FILE_CHANGED", (event) => {
        console.log("event: " + event.event + ", " + event.payload);
    })
}

window.invoke = async (name, args) => {
    const result = await invoke(name, args);
    console.log("buffer: " + result);
    return result;
}

window.fs_exists = async (path) => {
    return await fs.exists(path, {baseDir: 6});
}

window.fs_readTextFile = async (path) => {
    return await fs.readTextFile(path, {baseDir: 6});
}

window.fs_readDir = async (path) => {
    let files = []
    await processEntriesRecursive(path);

    async function processEntriesRecursive(path) {
        const entries = await fs.readDir(path, {baseDir: 6})

        for (const entry of entries) {
            const dir = path + "\\" + entry.name

            if (entry.isDirectory) {
                await processEntriesRecursive(dir)
            }

            if (entry.isFile) {
                files.push(dir)
            }
        }
    }

    return files;
}

window.fs_readFile = async (path) => {
    return await fs.readFile(path, {baseDir: 6});
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
