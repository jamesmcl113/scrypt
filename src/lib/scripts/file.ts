import { documentDir } from '@tauri-apps/api/path';
import { readTextFile, createDir, exists, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api';

/*
export const openFile = async (fileName: string) => {
	let contents = '';
	try {
		const docFolder = (await documentDir()) + 'scrypt/';

		// create document folder if it doesn't exist
		if (!(await exists(docFolder))) {
			await createDir(docFolder, { recursive: true });
		}

		// create file if it doesn't exist
		const filePath = docFolder + fileName;
		if (!(await exists(filePath))) {
			await writeTextFile(filePath, '');
		}

		contents = await readTextFile(filePath);
	} catch (e) {
		console.log(e);
	}

	return contents;
};
*/

interface FileResponse {
	contents: string;
	path: string;
}

const openFileDialog = async (): Promise<FileResponse> => {
	return await invoke('open_file');
};

const saveFile = async (path: string, contents: string) => {
	return await invoke('save_file', { path: path, content: contents });
};

const newFile = async (): Promise<FileResponse> => {
	return await invoke('new_file');
};

export class File {
	public path: string;
	public buffer: string;

	constructor(path: string, contents: string) {
		this.path = path;
		this.buffer = contents;
	}

	public async save() {
		await saveFile(this.path, this.buffer);
	}

	public static async init(): Promise<File> {
		const file = await openFileDialog();
		return new File(file.path, file.contents);
	}

	public static async init_empty(): Promise<File> {
		const file = await newFile();
		return new File(file.path, file.contents);
	}
}
