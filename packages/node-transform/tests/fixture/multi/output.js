const __ice_import_0__ = await __ice_import__("test1");
const foo = __ice_import_0__.default;
const __ice_import_1__ = await __ice_import__("test2");
const bar = __ice_import_1__.default;
console.log(foo, bar);