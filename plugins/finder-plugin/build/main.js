"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FinderPlugin = void 0;
var FinderPlugin = /** @class */ (function () {
    function FinderPlugin(name) {
        this.name = name;
    }
    FinderPlugin.prototype.execute = function () {
        console.log("Hello from the finder plugin!");
    };
    return FinderPlugin;
}());
exports.FinderPlugin = FinderPlugin;
