const event = window.__TAURI__.event;

const fileChangedEvent = "FileChanged";

export async function initialize(ref) {
	console.log("fileWatcher loaded");

	const unlisten = await event.listen(fileChangedEvent, (event) => {
		ref.invokeMethodAsync(fileChangedEvent, event.payload);
	});
}
