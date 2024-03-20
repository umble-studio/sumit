import {Plugin} from "sumit-plugin-interface";

export class FinderPlugin implements Plugin {
    name: string;

    constructor(name: string) {
        this.name = name;
    }

    execute() {
        console.log("Hello from the finder plugin!");
    }
}