/* eslint-disable */
var Module = typeof Module !== "undefined" ? Module : {};
var moduleOverrides = {};
var key;
for (key in Module) {
    if (Module.hasOwnProperty(key)) {
        moduleOverrides[key] = Module[key];
    }
}
var arguments_ = [];
var thisProgram = "./this.program";
var quit_ = function (status, toThrow) {
    throw toThrow;
};
var ENVIRONMENT_IS_WEB = typeof window === "object";
var ENVIRONMENT_IS_WORKER = typeof importScripts === "function";
var ENVIRONMENT_IS_NODE = typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string";
var scriptDirectory = "";
function locateFile(path) {
    if (Module["locateFile"]) {
        return Module["locateFile"](path, scriptDirectory);
    }
    return scriptDirectory + path;
}
var read_, readAsync, readBinary, setWindowTitle;
function logExceptionOnExit(e) {
    if (e instanceof ExitStatus) return;
    var toLog = e;
    err("exiting due to exception: " + toLog);
}
var nodeFS;
var nodePath;
if (ENVIRONMENT_IS_NODE) {
    if (ENVIRONMENT_IS_WORKER) {
        scriptDirectory = require("path").dirname(scriptDirectory) + "/";
    } else {
        scriptDirectory = __dirname + "/";
    }
    read_ = function shell_read(filename, binary) {
        console.log("hit")
        //if (!nodeFS) nodeFS = require("fs");
        //if (!nodePath) nodePath = require("path");
        filename = nodePath["normalize"](filename);
        return nodeFS["readFileSync"](filename, binary ? null : "utf8");
    };
    readBinary = function readBinary(filename) {
        var ret = read_(filename, true);
        if (!ret.buffer) {
            ret = new Uint8Array(ret);
        }
        assert(ret.buffer);
        return ret;
    };
    readAsync = function readAsync(filename, onload, onerror) {
        console.log("hit")
        //if (!nodeFS) nodeFS = require("fs");
        //if (!nodePath) nodePath = require("path");
        filename = nodePath["normalize"](filename);
        nodeFS["readFile"](filename, function (err, data) {
            if (err) onerror(err);
            else onload(data.buffer);
        });
    };
    if (process["argv"].length > 1) {
        thisProgram = process["argv"][1].replace(/\\/g, "/");
    }
    arguments_ = process["argv"].slice(2);
    if (typeof module !== "undefined") {
        module["exports"] = Module;
    }
    process["on"]("uncaughtException", function (ex) {
        if (!(ex instanceof ExitStatus)) {
            throw ex;
        }
    });
    process["on"]("unhandledRejection", function (reason) {
        throw reason;
    });
    quit_ = function (status, toThrow) {
        if (keepRuntimeAlive()) {
            process["exitCode"] = status;
            throw toThrow;
        }
        logExceptionOnExit(toThrow);
        process["exit"](status);
    };
    Module["inspect"] = function () {
        return "[Emscripten Module object]";
    };
} else if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_WORKER) {
    if (ENVIRONMENT_IS_WORKER) {
        scriptDirectory = self.location.href;
    } else if (typeof document !== "undefined" && document.currentScript) {
        scriptDirectory = document.currentScript.src;
    }
    if (scriptDirectory.indexOf("blob:") !== 0) {
        scriptDirectory = scriptDirectory.substr(0, scriptDirectory.replace(/[?#].*/, "").lastIndexOf("/") + 1);
    } else {
        scriptDirectory = "";
    }
    {
        read_ = function (url) {
            var xhr = new XMLHttpRequest();
            xhr.open("GET", url, false);
            xhr.send(null);
            return xhr.responseText;
        };
        if (ENVIRONMENT_IS_WORKER) {
            readBinary = function (url) {
                var xhr = new XMLHttpRequest();
                xhr.open("GET", url, false);
                xhr.responseType = "arraybuffer";
                xhr.send(null);
                return new Uint8Array(xhr.response);
            };
        }
        readAsync = function (url, onload, onerror) {
            var xhr = new XMLHttpRequest();
            xhr.open("GET", url, true);
            xhr.responseType = "arraybuffer";
            xhr.onload = function () {
                if (xhr.status == 200 || (xhr.status == 0 && xhr.response)) {
                    onload(xhr.response);
                    return;
                }
                onerror();
            };
            xhr.onerror = onerror;
            xhr.send(null);
        };
    }
    setWindowTitle = function (title) {
        document.title = title;
    };
} else {
}
var out = Module["print"] || console.log.bind(console);
var err = Module["printErr"] || console.warn.bind(console);
for (key in moduleOverrides) {
    if (moduleOverrides.hasOwnProperty(key)) {
        Module[key] = moduleOverrides[key];
    }
}
moduleOverrides = null;
if (Module["arguments"]) arguments_ = Module["arguments"];
if (Module["thisProgram"]) thisProgram = Module["thisProgram"];
if (Module["quit"]) quit_ = Module["quit"];
var wasmBinary;
if (Module["wasmBinary"]) wasmBinary = Module["wasmBinary"];
var noExitRuntime = Module["noExitRuntime"] || true;
if (typeof WebAssembly !== "object") {
    abort("no native wasm support detected");
}
var wasmMemory;
var ABORT = false;
var EXITSTATUS;
function assert(condition, text) {
    if (!condition) {
        abort("Assertion failed: " + text);
    }
}
function stringToUTF8Array(str, heap, outIdx, maxBytesToWrite) {
    if (!(maxBytesToWrite > 0)) return 0;
    var startIdx = outIdx;
    var endIdx = outIdx + maxBytesToWrite - 1;
    for (var i = 0; i < str.length; ++i) {
        var u = str.charCodeAt(i);
        if (u >= 55296 && u <= 57343) {
            var u1 = str.charCodeAt(++i);
            u = (65536 + ((u & 1023) << 10)) | (u1 & 1023);
        }
        if (u <= 127) {
            if (outIdx >= endIdx) break;
            heap[outIdx++] = u;
        } else if (u <= 2047) {
            if (outIdx + 1 >= endIdx) break;
            heap[outIdx++] = 192 | (u >> 6);
            heap[outIdx++] = 128 | (u & 63);
        } else if (u <= 65535) {
            if (outIdx + 2 >= endIdx) break;
            heap[outIdx++] = 224 | (u >> 12);
            heap[outIdx++] = 128 | ((u >> 6) & 63);
            heap[outIdx++] = 128 | (u & 63);
        } else {
            if (outIdx + 3 >= endIdx) break;
            heap[outIdx++] = 240 | (u >> 18);
            heap[outIdx++] = 128 | ((u >> 12) & 63);
            heap[outIdx++] = 128 | ((u >> 6) & 63);
            heap[outIdx++] = 128 | (u & 63);
        }
    }
    heap[outIdx] = 0;
    return outIdx - startIdx;
}
function lengthBytesUTF8(str) {
    var len = 0;
    for (var i = 0; i < str.length; ++i) {
        var u = str.charCodeAt(i);
        if (u >= 55296 && u <= 57343) u = (65536 + ((u & 1023) << 10)) | (str.charCodeAt(++i) & 1023);
        if (u <= 127) ++len;
        else if (u <= 2047) len += 2;
        else if (u <= 65535) len += 3;
        else len += 4;
    }
    return len;
}
function allocateUTF8OnStack(str) {
    var size = lengthBytesUTF8(str) + 1;
    var ret = stackAlloc(size);
    stringToUTF8Array(str, HEAP8, ret, size);
    return ret;
}
var buffer, HEAP8, HEAPU8, HEAP16, HEAPU16, HEAP32, HEAPU32, HEAPF32, HEAPF64;
function updateGlobalBufferAndViews(buf) {
    buffer = buf;
    Module["HEAP8"] = HEAP8 = new Int8Array(buf);
    Module["HEAP16"] = HEAP16 = new Int16Array(buf);
    Module["HEAP32"] = HEAP32 = new Int32Array(buf);
    Module["HEAPU8"] = HEAPU8 = new Uint8Array(buf);
    Module["HEAPU16"] = HEAPU16 = new Uint16Array(buf);
    Module["HEAPU32"] = HEAPU32 = new Uint32Array(buf);
    Module["HEAPF32"] = HEAPF32 = new Float32Array(buf);
    Module["HEAPF64"] = HEAPF64 = new Float64Array(buf);
}
var INITIAL_MEMORY = Module["INITIAL_MEMORY"] || 16777216;
var wasmTable;
var __ATPRERUN__ = [];
var __ATINIT__ = [];
var __ATMAIN__ = [];
var __ATPOSTRUN__ = [];
var runtimeInitialized = false;
var runtimeExited = false;
var runtimeKeepaliveCounter = 0;
function keepRuntimeAlive() {
    return noExitRuntime || runtimeKeepaliveCounter > 0;
}
function preRun() {
    if (Module["preRun"]) {
        if (typeof Module["preRun"] == "function") Module["preRun"] = [Module["preRun"]];
        while (Module["preRun"].length) {
            addOnPreRun(Module["preRun"].shift());
        }
    }
    callRuntimeCallbacks(__ATPRERUN__);
}
function initRuntime() {
    runtimeInitialized = true;
    callRuntimeCallbacks(__ATINIT__);
}
function preMain() {
    callRuntimeCallbacks(__ATMAIN__);
}
function exitRuntime() {
    runtimeExited = true;
}
function postRun() {
    if (Module["postRun"]) {
        if (typeof Module["postRun"] == "function") Module["postRun"] = [Module["postRun"]];
        while (Module["postRun"].length) {
            addOnPostRun(Module["postRun"].shift());
        }
    }
    callRuntimeCallbacks(__ATPOSTRUN__);
}
function addOnPreRun(cb) {
    __ATPRERUN__.unshift(cb);
}
function addOnInit(cb) {
    __ATINIT__.unshift(cb);
}
function addOnPostRun(cb) {
    __ATPOSTRUN__.unshift(cb);
}
var runDependencies = 0;
var runDependencyWatcher = null;
var dependenciesFulfilled = null;
function addRunDependency(id) {
    runDependencies++;
    if (Module["monitorRunDependencies"]) {
        Module["monitorRunDependencies"](runDependencies);
    }
}
function removeRunDependency(id) {
    runDependencies--;
    if (Module["monitorRunDependencies"]) {
        Module["monitorRunDependencies"](runDependencies);
    }
    if (runDependencies == 0) {
        if (runDependencyWatcher !== null) {
            clearInterval(runDependencyWatcher);
            runDependencyWatcher = null;
        }
        if (dependenciesFulfilled) {
            var callback = dependenciesFulfilled;
            dependenciesFulfilled = null;
            callback();
        }
    }
}
Module["preloadedImages"] = {};
Module["preloadedAudios"] = {};
function abort(what) {
    {
        if (Module["onAbort"]) {
            Module["onAbort"](what);
        }
    }
    what = "Aborted(" + what + ")";
    err(what);
    ABORT = true;
    EXITSTATUS = 1;
    what += ". Build with -s ASSERTIONS=1 for more info.";
    var e = new WebAssembly.RuntimeError(what);
    throw e;
}
var dataURIPrefix = "data:application/octet-stream;base64,";
function isDataURI(filename) {
    return filename.startsWith(dataURIPrefix);
}
function isFileURI(filename) {
    return filename.startsWith("file://");
}
var wasmBinaryFile;
wasmBinaryFile = "assets/emulator.wasm";
if (!isDataURI(wasmBinaryFile)) {
    wasmBinaryFile = locateFile(wasmBinaryFile);
}
function getBinary(file) {
    try {
        if (file == wasmBinaryFile && wasmBinary) {
            return new Uint8Array(wasmBinary);
        }
        if (readBinary) {
            return readBinary(file);
        } else {
            throw "both async and sync fetching of the wasm failed";
        }
    } catch (err) {
        abort(err);
    }
}
function getBinaryPromise() {
    if (!wasmBinary && (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_WORKER)) {
        if (typeof fetch === "function" && !isFileURI(wasmBinaryFile)) {
            return fetch(wasmBinaryFile, { credentials: "same-origin" })
                .then(function (response) {
                    if (!response["ok"]) {
                        throw "failed to load wasm binary file at '" + wasmBinaryFile + "'";
                    }
                    return response["arrayBuffer"]();
                })
                .catch(function () {
                    return getBinary(wasmBinaryFile);
                });
        } else {
            if (readAsync) {
                return new Promise(function (resolve, reject) {
                    readAsync(
                        wasmBinaryFile,
                        function (response) {
                            resolve(new Uint8Array(response));
                        },
                        reject
                    );
                });
            }
        }
    }
    return Promise.resolve().then(function () {
        return getBinary(wasmBinaryFile);
    });
}
function createWasm() {
    var info = { a: asmLibraryArg };
    function receiveInstance(instance, module) {
        var exports = instance.exports;
        Module["asm"] = exports;
        wasmMemory = Module["asm"]["b"];
        updateGlobalBufferAndViews(wasmMemory.buffer);
        wasmTable = Module["asm"]["e"];
        addOnInit(Module["asm"]["c"]);
        removeRunDependency("wasm-instantiate");
    }
    addRunDependency("wasm-instantiate");
    function receiveInstantiationResult(result) {
        receiveInstance(result["instance"]);
    }
    function instantiateArrayBuffer(receiver) {
        return getBinaryPromise()
            .then(function (binary) {
                return WebAssembly.instantiate(binary, info);
            })
            .then(function (instance) {
                return instance;
            })
            .then(receiver, function (reason) {
                err("failed to asynchronously prepare wasm: " + reason);
                abort(reason);
            });
    }
    function instantiateAsync() {
        if (!wasmBinary && typeof WebAssembly.instantiateStreaming === "function" && !isDataURI(wasmBinaryFile) && !isFileURI(wasmBinaryFile) && typeof fetch === "function") {
            return fetch(wasmBinaryFile, { credentials: "same-origin" }).then(function (response) {
                var result = WebAssembly.instantiateStreaming(response, info);
                return result.then(receiveInstantiationResult, function (reason) {
                    err("wasm streaming compile failed: " + reason);
                    err("falling back to ArrayBuffer instantiation");
                    return instantiateArrayBuffer(receiveInstantiationResult);
                });
            });
        } else {
            return instantiateArrayBuffer(receiveInstantiationResult);
        }
    }
    if (Module["instantiateWasm"]) {
        try {
            var exports = Module["instantiateWasm"](info, receiveInstance);
            return exports;
        } catch (e) {
            err("Module.instantiateWasm callback failed with error: " + e);
            return false;
        }
    }
    instantiateAsync();
    return {};
}
function callRuntimeCallbacks(callbacks) {
    while (callbacks.length > 0) {
        var callback = callbacks.shift();
        if (typeof callback == "function") {
            callback(Module);
            continue;
        }
        var func = callback.func;
        if (typeof func === "number") {
            if (callback.arg === undefined) {
                getWasmTableEntry(func)();
            } else {
                getWasmTableEntry(func)(callback.arg);
            }
        } else {
            func(callback.arg === undefined ? null : callback.arg);
        }
    }
}
var wasmTableMirror = [];
function getWasmTableEntry(funcPtr) {
    var func = wasmTableMirror[funcPtr];
    if (!func) {
        if (funcPtr >= wasmTableMirror.length) wasmTableMirror.length = funcPtr + 1;
        wasmTableMirror[funcPtr] = func = wasmTable.get(funcPtr);
    }
    return func;
}
function handleException(e) {
    if (e instanceof ExitStatus || e == "unwind") {
        return EXITSTATUS;
    }
    quit_(1, e);
}
function abortOnCannotGrowMemory(requestedSize) {
    abort("OOM");
}
function _emscripten_resize_heap(requestedSize) {
    var oldSize = HEAPU8.length;
    requestedSize = requestedSize >>> 0;
    abortOnCannotGrowMemory(requestedSize);
}
var asmLibraryArg = { a: _emscripten_resize_heap };
var asm = createWasm();
var ___wasm_call_ctors = (Module["___wasm_call_ctors"] = function () {
    return (___wasm_call_ctors = Module["___wasm_call_ctors"] = Module["asm"]["c"]).apply(null, arguments);
});
var _main = (Module["_main"] = function () {
    return (_main = Module["_main"] = Module["asm"]["d"]).apply(null, arguments);
});
var stackAlloc = (Module["stackAlloc"] = function () {
    return (stackAlloc = Module["stackAlloc"] = Module["asm"]["f"]).apply(null, arguments);
});
var calledRun;
function ExitStatus(status) {
    this.name = "ExitStatus";
    this.message = "Program terminated with exit(" + status + ")";
    this.status = status;
}
var calledMain = false;
dependenciesFulfilled = function runCaller() {
    if (!calledRun) run();
    if (!calledRun) dependenciesFulfilled = runCaller;
};
function callMain(args) {
    var entryFunction = Module["_main"];
    args = args || [];
    var argc = args.length + 1;
    var argv = stackAlloc((argc + 1) * 4);
    HEAP32[argv >> 2] = allocateUTF8OnStack(thisProgram);
    for (var i = 1; i < argc; i++) {
        HEAP32[(argv >> 2) + i] = allocateUTF8OnStack(args[i - 1]);
    }
    HEAP32[(argv >> 2) + argc] = 0;
    try {
        var ret = entryFunction(argc, argv);
        exit(ret, true);
        return ret;
    } catch (e) {
        return handleException(e);
    } finally {
        calledMain = true;
    }
}
function run(args) {
    args = args || arguments_;
    if (runDependencies > 0) {
        return;
    }
    preRun();
    if (runDependencies > 0) {
        return;
    }
    function doRun() {
        if (calledRun) return;
        calledRun = true;
        Module["calledRun"] = true;
        if (ABORT) return;
        initRuntime();
        preMain();
        if (Module["onRuntimeInitialized"]) Module["onRuntimeInitialized"]();
        if (shouldRunNow) callMain(args);
        postRun();
    }
    if (Module["setStatus"]) {
        Module["setStatus"]("Running...");
        setTimeout(function () {
            setTimeout(function () {
                Module["setStatus"]("");
            }, 1);
            doRun();
        }, 1);
    } else {
        doRun();
    }
}
Module["run"] = run;
function exit(status, implicit) {
    EXITSTATUS = status;
    if (keepRuntimeAlive()) {
    } else {
        exitRuntime();
    }
    procExit(status);
}
function procExit(code) {
    EXITSTATUS = code;
    if (!keepRuntimeAlive()) {
        if (Module["onExit"]) Module["onExit"](code);
        ABORT = true;
    }
    quit_(code, new ExitStatus(code));
}
if (Module["preInit"]) {
    if (typeof Module["preInit"] == "function") Module["preInit"] = [Module["preInit"]];
    while (Module["preInit"].length > 1) {
        Module["preInit"].pop()();
    }
}
var shouldRunNow = true;
if (Module["noInitialRun"]) shouldRunNow = false;
run();