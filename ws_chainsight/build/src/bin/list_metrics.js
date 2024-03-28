"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const principal_1 = require("@dfinity/principal");
const common_1 = require("../common");
BigInt.prototype.toJSON = function () {
    return this.toString();
};
(0, common_1.bootstrap)();
const execute = async () => {
    const components = await (0, common_1.loadJson)(common_1.PATH_COMPONENTS);
    const sliced = components.slice(0, components.length < 20 ? components.length : 20);
    const agent = (0, common_1.getAgent)();
    const result = [];
    for (const c of sliced) {
        const principal = principal_1.Principal.fromText(c.vault);
        const actor = (0, common_1.vaultActor)(principal, agent);
        const metric = await actor.metric().catch(() => {
            return null;
        });
        const data = {
            id: c.id,
            label: c.label,
            metric,
        };
        result.push(data);
    }
    console.log(result);
    // writeJson(PATH_METRIC, result);
};
execute()
    .then(() => {
    console.log('Execution completed');
})
    .catch((error) => {
    console.error(error);
});
//# sourceMappingURL=list_metrics.js.map