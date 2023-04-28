"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const agent_1 = require("@dfinity/agent");
// import * as measurer from "./declarations/measurer";
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const agent = new agent_1.HttpAgent({
            host: 'http://127.0.0.1:4943',
            fetch: require('node-fetch')
        });
        if (process.env.NODE_ENV !== "production")
            yield agent.fetchRootKey();
    });
}
main();
