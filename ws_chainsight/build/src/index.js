"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const aws_amplify_1 = require("aws-amplify");
const aws_exports_1 = require("./aws-exports");
aws_amplify_1.Amplify.configure(aws_exports_1.default);
const execute = async () => {
    console.log('Hello World');
};
execute()
    .then(() => {
    console.log('Execution completed');
})
    .catch((error) => {
    console.error(error);
});
//# sourceMappingURL=index.js.map