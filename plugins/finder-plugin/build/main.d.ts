import { Plugin } from "sumit-plugin-interface";
export declare class FinderPlugin implements Plugin {
    name: string;
    constructor(name: string);
    execute(): void;
}
