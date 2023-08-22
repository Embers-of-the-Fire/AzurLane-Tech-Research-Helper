"use strict";
/* eslint-disable */

const NAME_LIST = [
    "魔方解析1h",
    "魔方解析2h",
    "魔方解析4h",
    "魔方解析0.5h",
    "舰装解析1h",
    "舰装解析2h",
    "舰装解析4h",
    "舰装解析0.5h",
    "金船定向2.5h",
    "金船定向5h",
    "金船定向8h",
    "金船定向0.5h",
    "彩船定向2.5h",
    "彩船定向5h",
    "彩船定向8h",
    "彩船定向0.5h",
    "资金募集1.5h",
    "资金募集2.5h",
    "资金募集4h",
    "蓝试验品募集2h",
    "紫试验品募集2h",
    "紫数据收集4h",
    "金数据收集4h",
    "基础研究6h",
    "基础研究8h",
    "基础研究12h",
    "研究委托3h",
    "研究委托4h",
    "研究委托6h",
];
const TRUE_IMG = "&#10004;";
const FALSE_IMG = "&#10006;";
let resplan_situation = true; // true for daily and false for average
let higher_level_option = false; // true for on and false for off
let predict_situation = 0; // 0 for "全年产出", 1 for "科研船从零满破", 2 for "直到六期开始"
let restriction, resplan, referenceValue;
const END_DATE = new Date("2024/7/13 9:00:00");

const wasm_handler = (function () {
    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) {
        return heap[idx];
    }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    let cachedFloat64Memory0 = new Float64Array();

    function getFloat64Memory0() {
        if (cachedFloat64Memory0.byteLength === 0) {
            cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
        }
        return cachedFloat64Memory0;
    }

    let cachedInt32Memory0 = new Int32Array();

    function getInt32Memory0() {
        if (cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    const cachedTextDecoder = new TextDecoder("utf-8", {
        ignoreBOM: true,
        fatal: true,
    });

    cachedTextDecoder.decode();

    let cachedUint8Memory0 = new Uint8Array();

    function getUint8Memory0() {
        if (cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(
            getUint8Memory0().subarray(ptr, ptr + len)
        );
    }

    let WASM_VECTOR_LEN = 0;

    const cachedTextEncoder = new TextEncoder("utf-8");

    const encodeString =
        typeof cachedTextEncoder.encodeInto === "function"
            ? function (arg, view) {
                  return cachedTextEncoder.encodeInto(arg, view);
              }
            : function (arg, view) {
                  const buf = cachedTextEncoder.encode(arg);
                  view.set(buf);
                  return {
                      read: arg.length,
                      written: buf.length,
                  };
              };

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0()
                .subarray(ptr, ptr + buf.length)
                .set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7f) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, (len = offset + arg.length * 3));
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == "number" || type == "boolean" || val == null) {
            return `${val}`;
        }
        if (type == "string") {
            return `"${val}"`;
        }
        if (type == "symbol") {
            const description = val.description;
            if (description == null) {
                return "Symbol";
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == "function") {
            const name = val.name;
            if (typeof name == "string" && name.length > 0) {
                return `Function(${name})`;
            } else {
                return "Function";
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = "[";
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for (let i = 1; i < length; i++) {
                debug += ", " + debugString(val[i]);
            }
            debug += "]";
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == "Object") {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return "Object(" + JSON.stringify(val) + ")";
            } catch (_) {
                return "Object";
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    /**
     * @returns {any}
     */
    function predef_restriction() {
        const ret = wasm.predef_restriction();
        return takeObject(ret);
    }

    /**
     * @returns {any}
     */
    function predef_reference_value() {
        const ret = wasm.predef_reference_value();
        return takeObject(ret);
    }

    /**
     * @param {number} doubloon_ratio
     * @param {number} cube_ratio
     * @param {number} cong_chips_ratio
     * @param {number} time_ratio
     * @param {number} ultra_blp_ratio
     * @param {number} ultra_equip_ratio
     * @param {number} fni_5_super_r
     * @param {number} fni_5_ultra_r
     * @param {number} fni_f
     * @param {boolean} do_data_collection
     * @param {boolean} do_research_assignment
     * @returns {any}
     */
    function build_restriction(
        doubloon_ratio,
        cube_ratio,
        cong_chips_ratio,
        time_ratio,
        ultra_blp_ratio,
        ultra_equip_ratio,
        fni_5_super_r,
        fni_5_ultra_r,
        fni_f,
        do_data_collection,
        do_research_assignment
    ) {
        const ret = wasm.build_restriction(
            doubloon_ratio,
            cube_ratio,
            cong_chips_ratio,
            time_ratio,
            ultra_blp_ratio,
            ultra_equip_ratio,
            fni_5_super_r,
            fni_5_ultra_r,
            fni_f,
            do_data_collection,
            do_research_assignment
        );
        return takeObject(ret);
    }

    /**
     * @param {number} doubloon
     * @param {number} cube
     * @param {number} time_of_an_hour
     * @param {number} super_rare
     * @param {number} ultra_rare
     * @param {number} ultra_equip
     * @param {number} cong_chips
     * @param {number} time_ratio
     * @returns {any}
     */
    function build_reference_value(
        doubloon,
        cube,
        time_of_an_hour,
        super_rare,
        ultra_rare,
        ultra_equip,
        cong_chips,
        time_ratio
    ) {
        const ret = wasm.build_reference_value(
            doubloon,
            cube,
            time_of_an_hour,
            super_rare,
            ultra_rare,
            ultra_equip,
            cong_chips,
            time_ratio
        );
        return takeObject(ret);
    }

    /**
     * @param {any} rest
     * @param {any} raw_ref
     * @param {number} refer_v
     * @param {number} limit
     * @returns {any}
     */
    function calc(rest, raw_ref, refer_v, limit) {
        const ret = wasm.calc(
            addHeapObject(rest),
            addHeapObject(raw_ref),
            refer_v,
            limit
        );
        return takeObject(ret);
    }

    async function load(module, imports) {
        if (typeof Response === "function" && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === "function") {
                try {
                    return await WebAssembly.instantiateStreaming(
                        module,
                        imports
                    );
                } catch (e) {
                    if (
                        module.headers.get("Content-Type") != "application/wasm"
                    ) {
                        console.warn(
                            "`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
                            e
                        );
                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);
        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };
            } else {
                return instance;
            }
        }
    }

    function getImports() {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbindgen_object_drop_ref = function (arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbg_alert_9ddce09516a12948 = function (arg0, arg1) {
            alert(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_is_undefined = function (arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        };
        imports.wbg.__wbindgen_in = function (arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        };
        imports.wbg.__wbindgen_number_get = function (arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "number" ? obj : undefined;
            getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        };
        imports.wbg.__wbindgen_boolean_get = function (arg0) {
            const v = getObject(arg0);
            const ret = typeof v === "boolean" ? (v ? 1 : 0) : 2;
            return ret;
        };
        imports.wbg.__wbindgen_is_object = function (arg0) {
            const val = getObject(arg0);
            const ret = typeof val === "object" && val !== null;
            return ret;
        };
        imports.wbg.__wbindgen_number_new = function (arg0) {
            const ret = arg0;
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_string_new = function (arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_object_clone_ref = function (arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_jsval_loose_eq = function (arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        };
        imports.wbg.__wbindgen_string_get = function (arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof obj === "string" ? obj : undefined;
            var ptr0 = isLikeNone(ret)
                ? 0
                : passStringToWasm0(
                      ret,
                      wasm.__wbindgen_malloc,
                      wasm.__wbindgen_realloc
                  );
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbindgen_error_new = function (arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_getwithrefkey_15c62c2b8546208d = function (
            arg0,
            arg1
        ) {
            const ret = getObject(arg0)[getObject(arg1)];
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_set_20cbc34131e76824 = function (arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        };
        imports.wbg.__wbg_new_1d9a920c6bfc44a8 = function () {
            const ret = new Array();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_new_0b9bfdd97583284e = function () {
            const ret = new Object();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_set_a68214f35c417fa9 = function (arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
        };
        imports.wbg.__wbg_instanceof_ArrayBuffer_e5e48f4762c5610b = function (
            arg0
        ) {
            let result;
            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_isSafeInteger_dfa0593e8d7ac35a = function (arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        };
        imports.wbg.__wbg_buffer_3f3d764d4747d564 = function (arg0) {
            const ret = getObject(arg0).buffer;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_new_8c3f0052272a457a = function (arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_set_83db9690f9353e79 = function (arg0, arg1, arg2) {
            getObject(arg0).set(getObject(arg1), arg2 >>> 0);
        };
        imports.wbg.__wbg_length_9e1ae1900cb0fbd5 = function (arg0) {
            const ret = getObject(arg0).length;
            return ret;
        };
        imports.wbg.__wbg_instanceof_Uint8Array_971eeda69eb75003 = function (
            arg0
        ) {
            let result;
            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch {
                result = false;
            }
            const ret = result;
            return ret;
        };
        imports.wbg.__wbg_new_abda76e883ba8a5f = function () {
            const ret = new Error();
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_stack_658279fe44541cf6 = function (arg0, arg1) {
            const ret = getObject(arg1).stack;
            const ptr0 = passStringToWasm0(
                ret,
                wasm.__wbindgen_malloc,
                wasm.__wbindgen_realloc
            );
            const len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbg_error_f851667af71bcfc6 = function (arg0, arg1) {
            try {
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(arg0, arg1);
            }
        };
        imports.wbg.__wbindgen_debug_string = function (arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr0 = passStringToWasm0(
                ret,
                wasm.__wbindgen_malloc,
                wasm.__wbindgen_realloc
            );
            const len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbindgen_throw = function (arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_memory = function () {
            const ret = wasm.memory;
            return addHeapObject(ret);
        };

        return imports;
    }

    function initMemory(imports, maybe_memory) {}

    function finalizeInit(instance, module) {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        cachedFloat64Memory0 = new Float64Array();
        cachedInt32Memory0 = new Int32Array();
        cachedUint8Memory0 = new Uint8Array();

        return wasm;
    }

    function initSync(module) {
        const imports = getImports();

        initMemory(imports);

        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }

        const instance = new WebAssembly.Instance(module, imports);

        return finalizeInit(instance, module);
    }

    async function init(input) {
        const imports = getImports();

        if (
            typeof input === "string" ||
            (typeof Request === "function" && input instanceof Request) ||
            (typeof URL === "function" && input instanceof URL)
        ) {
            input = fetch(input);
        }

        initMemory(imports);

        const { instance, module } = await load(await input, imports);

        return finalizeInit(instance, module);
    }
    return {
        init: init,
        calc: calc,
        build_reference_value: build_reference_value,
        build_restriction: build_restriction,
        predef_restriction: predef_restriction,
        predef_reference_value: predef_reference_value,
    };
})();

const init = wasm_handler.init;
const calc = wasm_handler.calc;
const build_reference_value = wasm_handler.build_reference_value;
const build_restriction = wasm_handler.build_restriction;

init("./azurlane_tech_research_bg.wasm");

window.onload = function () {
    document.getElementById("render-alloc").innerHTML =
        '<div class="title row col-12"><p class="h1">碧蓝航线科研规划器</p><br><sup>AzurLane Tech Research Helper</sup><br><a href="https://github.com/Embers-of-the-Fire/AzurLane-Tech-Research-Helper"><img src="https://img.shields.io/badge/Repo-%20AzurLane--Tech--Research--Helper-success"></a><div class="divider"><hr class="simple"/></div></div><div class="form row col-12"><div class="form-surrounder col-12 col-md-12 col-xl-4"id="rest-surrounder"><h3>限制条件&nbsp;&nbsp;<button id="high_level_option"class="btn btn-outline-primary">高级选项-已关闭</button></h3><form id="RestrictionForm"class="needs-validation"novalidate><div class="row fsur"><div class="mb-3 mt-3 col-6"><div class="mb-3 mt-3"><label for="doubloon_rest"class="form-label">省物资倾向</label><input type="number"class="form-control"id="doubloon_rest"name="doubloon_rest"value="0"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="cube_rest"class="form-label">省魔方倾向</label><input type="number"class="form-control"id="cube_rest"name="cube_rest"value="110"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="cogn_chips_rest"class="form-label">缺心智倾向</label><input type="number"name="cogn_chips_rest"value="0",id="cogn_chips_rest"step="0.01"class="form-control"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="time_rest"class="form-label">当咸鱼倾向</label><input type="number"class="form-control"value="0"id="time_rest"name="time_rest"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="urship_value"class="form-label"><abbr title="1彩科研船图纸=n金科研船图纸">彩船图纸价值</abbr></label><input type="number"class="form-control"value="3"id="urship_value"name="urship_value"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label class="form-check-label"><input class="form-check-input"type="checkbox"name="do_data_collection">做数据收集</label></div></div><div class="mb-3 mt-3 col-6"><div class="mb-3 mt-3"><label for="urequip_value"class="form-label"><abbr title="1彩科研装备图纸价值=n彩科研船图纸">彩装图纸价值</abbr></label><input type="number"class="form-control"value="20"id="urequip_value"name="urequip_value"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="finish_5_ssr"class="form-label">6期毕业金船数</label><input type="number"class="form-control"id="finish_5_ssr"name="finish_5_ssr"value="3"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="finish_5_ur"class="form-label">6期毕业彩船数</label><input type="number"class="form-control"id="finish_5_ur"name="finish_5_ur"value="2"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="finish_uship"class="form-label">345期毕业船数</label><input type="number"class="form-control"id="finish_uship"name="finish_uship"value="11"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label for="limit"class="form-label"><abbr title="过滤掉排名大于N的项目">忽略项目</abbr></label><input type="number"class="form-control"id="limit"name="limit"value="10"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3"><label class="form-check-label"><input class="form-check-input"type="checkbox"name="do_research_assignment">做研究委托</label></div></div></div></form></div><div class="form-surrounder col-12 col-md-6 col-xl-2 d-none"id="refer-surrounder"><h3>参考价值</h3><form id="ReferenceForm"class="needs-validation"novalidate><div class="row fsur"><div class="mb-3 mt-3 col-12"><div class="row"><div class="mb-3 mt-3 col"><label for="doubloon_refer"class="form-label">物资</label><input type="number"class="form-control"id="doubloon_refer"value="0.5"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3 col"><label for="cube_refer"class="form-label">魔方</label><input type="number"class="form-control"name="cube_refer"value="1000"id="cube_refer"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div></div><div class="row"><div class="mb-3 mt-3 col"><label for="time_refer"class="form-label">1H时间</label><input type="number"class="form-control"name="time_refer"value="1500"id="time_refer"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3 col"><label for="ssr_refer"class="form-label">金图</label><input type="number"class="form-control"name="ssr_refer"value="1200"id="ssr_refer"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div></div><div class="row"><div class="mb-3 mt-3 col"><label for="ur_refer"class="form-label">彩图</label><input type="number"class="form-control"name="ur_refer"value="3000"id="ur_refer"step="0.01"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3 col"><label for="ur_equip_refer"class="form-label">彩装</label><input type="number"class="form-control"name="ur_equip_refer"value="36000"step="0.01"id="ur_equip_refer"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div></div><div class="row"><div class="mb-3 mt-3 col"><label for="cogn_chips_refer"class="form-label">心智单元</label><input type="number"class="form-control"name="cogn_chips_refer"value="60"step="0.01"id="cogn_chips_refer"required><div class="invalid-feedback"></div><div class="valid-feedback"></div></div><div class="mb-3 mt-3 col"><label for="start_refer"class="form-label">迭代参考值</label><input type="number"class="form-control"id="start_refer"name="start_refer"step="0.00001"value="150"><div class="invalid-feedback"></div><div class="valid-feedback"></div></div></div></div></div></form></div><style>@media(min-width:1200px){div.tech-table-surrounder,div.result-table-surrounder,div.predict-table-surrounder{max-height:480px;padding-right:0px}div.result-surrounder{max-height:520px;padding-right:0px}}@media(max-width:1200px){div.tech-table-surrounder,div.result-table-surrounder,div.predict-table-surrounder{padding-right:0px;max-height:70vh}div.result-surrounder{padding-right:0px}}</style><div class="result-surrounder col-12 col-xl-8 row"style="max-height: 520px;"id="result-surrounder"><h3 style="height: 40px;">模拟结果&nbsp;&nbsp;&nbsp;<button id="refresh"class="btn btn-primary">加载科研规划</button></h3><div class="tech-table-surrounder col-12 col-xl-4 overflow-auto"><table class="table table-bordered table-hover"><thead class="sticky-top"><tr class="table-light"><th>排名</th><th>科研名称</th><th>优先</th></tr></thead><tbody><tr><td>1</td><td>舰装解析0.5h</td><td>✔</td></tr><tr><td>2</td><td>彩船定向0.5h</td><td>✔</td></tr><tr><td>3</td><td>金船定向0.5h</td><td>✔</td></tr><tr><td>4</td><td>魔方解析0.5h</td><td>✔</td></tr><tr><td>5</td><td>彩船定向2.5h</td><td>✔</td></tr><tr><td>6</td><td>资金募集4h</td><td>✔</td></tr><tr><td>7</td><td>舰装解析1h</td><td>✔</td></tr><tr><td>8</td><td>舰装解析2h</td><td>✔</td></tr><tr><td>9</td><td>舰装解析4h</td><td>✔</td></tr><tr><td>10</td><td>资金募集1.5h</td><td>✔</td></tr><tr><td>11</td><td>金船定向2.5h</td><td>✖</td></tr><tr><td>12</td><td>彩船定向5h</td><td>✖</td></tr><tr><td>13</td><td>紫试验品募集2h</td><td>✖</td></tr><tr><td>14</td><td>资金募集2.5h</td><td>✖</td></tr><tr><td>15</td><td>蓝试验品募集2h</td><td>✖</td></tr><tr><td>16</td><td>彩船定向8h</td><td>✖</td></tr><tr><td>17</td><td>魔方解析1h</td><td>✖</td></tr><tr><td>18</td><td>金船定向5h</td><td>✖</td></tr><tr><td>19</td><td>金船定向8h</td><td>✖</td></tr><tr><td>20</td><td>魔方解析2h</td><td>✖</td></tr><tr><td>21</td><td>基础研究6h</td><td>✖</td></tr><tr><td>22</td><td>基础研究8h</td><td>✖</td></tr><tr><td>23</td><td>魔方解析4h</td><td>✖</td></tr><tr><td>24</td><td>基础研究12h</td><td>✖</td></tr><tr><td>25</td><td>研究委托3h</td><td>✖</td></tr><tr><td>26</td><td>金数据收集4h</td><td>✖</td></tr><tr><td>27</td><td>紫数据收集4h</td><td>✖</td></tr><tr><td>28</td><td>研究委托4h</td><td>✖</td></tr><tr><td>29</td><td>研究委托6h</td><td>✖</td></tr></tbody></table></div><div class="result-table-surrounder col-12 col-xl-4"style="max-height: 480px;"><h5 style="margin-top: 10px;"><a>科研理论效益&nbsp;&nbsp;</a><button type="button"class="btn btn-outline-primary"id="convert_restable">切换显示&nbsp;<i class="fa fa-refresh"></i></button></h5><table class="table table-bordered table-hover"><tbody></tbody></table></div><div class="predict-table-surrounder col-12 col-xl-4"style="max-height: 480px;"><h5 style="margin-top: 10px;"><a>预计科研产出&nbsp;&nbsp;</a><button type="button"class="btn btn-outline-primary"id="convert_predict_table">切换显示&nbsp;<i class="fa fa-refresh"></i></button></h5><table class="table table-bordered table-hover"><tbody></tbody></table></div></div></div>';
    document.getElementById("refresh").addEventListener("click", submit);
    document
        .getElementById("convert_restable")
        .addEventListener("click", convert_result_table);
    document
        .getElementById("convert_predict_table")
        .addEventListener("click", convert_predict_table);
    document
        .getElementById("high_level_option")
        .addEventListener("click", convert_higher_level_option);
};

function submit() {
    init();
    let formRes = document.getElementById("RestrictionForm");
    let formRef = document.getElementById("ReferenceForm");
    formRes.classList.add("was-validated");
    formRef.classList.add("was-validated");
    if (formRes.checkValidity() !== true || formRef.checkValidity() !== true)
        return;
    restriction = build_restriction(
        formRes.doubloon_rest.value,
        formRes.cube_rest.value,
        formRes.cogn_chips_rest.value,
        formRes.time_rest.value,
        formRes.urship_value.value,
        formRes.urequip_value.value,
        formRes.finish_5_ssr.value,
        formRes.finish_5_ur.value,
        formRes.finish_uship.value,
        formRes.do_data_collection.checked,
        formRes.do_research_assignment.checked
    );
    referenceValue = build_reference_value(
        formRef.doubloon_refer.value,
        formRef.cube_refer.value,
        formRef.time_refer.value,
        formRef.ssr_refer.value,
        formRef.ur_refer.value,
        formRef.ur_equip_refer.value,
        formRef.cogn_chips_refer.value,
        formRef.time_refer.value * 72
    );
    // restriction = predef_restriction();
    // referenceValue = predef_reference_value();
    let limit = formRes.limit.value;
    let fv = 150;
    if (formRef.start_refer.value) fv = formRef.start_refer.value;
    let former_refer_v, refer_v, former2_refer_v;
    former2_refer_v = fv;
    former_refer_v = fv;
    let lim = 0;
    while (true) {
        try {
            resplan = calc(restriction, referenceValue, former_refer_v, limit);
        } catch (err) {
            alert(
                "出错了！\n错误内容：" +
                    err +
                    "\n请修改入参后重试，如果仍然错误，请向作者报告"
            );
            return;
        }
        refer_v = resplan.result.cost_performance;
        if (Math.abs(refer_v - former_refer_v) <= 0.0001) break;
        else if (Math.abs(refer_v - former2_refer_v) <= 0.0001) {
            if (former_refer_v >= refer_v) calc(refer_v);
            break;
        }
        former2_refer_v = former_refer_v;
        former_refer_v = refer_v;
        lim += 1;
        if (lim == 50)
            alert(
                "迭代次数超过预设次数50次，直接保留最后一次结果，请尝试修改迭代初始值，或将最后一次结果的参考值作为初始值以继续迭代"
            );
    }
    // alert(resplan.result.cost_performance)
    let table_resproject = document.querySelector(
        "body div.form.row div.result-surrounder div.tech-table-surrounder table tbody"
    );
    table_resproject.innerHTML = "";
    let pl = resplan.projects.data;
    let nameid, name;
    let newtr, newtdindex, newtdname, newtdselect;
    for (var i = 0; i < pl.length; i++) {
        nameid = pl[i].data.data.research_id;
        name = NAME_LIST[nameid];
        newtdindex = document.createElement("td");
        newtdindex.innerHTML = i + 1;
        newtdname = document.createElement("td");
        newtdname.innerHTML = name;
        newtdselect = document.createElement("td");
        if (i < limit) newtdselect.innerHTML = TRUE_IMG;
        else newtdselect.innerHTML = FALSE_IMG;
        newtr = document.createElement("tr");
        newtr.appendChild(newtdindex);
        newtr.appendChild(newtdname);
        newtr.appendChild(newtdselect);
        table_resproject.appendChild(newtr);
    }
    convert_result_table_to_daily();
    predict_situation_0();
}

function string_formatter(str) {
    var args = arguments,
        re = new RegExp("%([1-" + args.length + "])", "g");
    return String(str).replace(re, function (_, $2) {
        return args[$2];
    });
}

function convert_result_table() {
    if (!resplan || !restriction || !referenceValue) {
        alert("请先刷新数据呢");
        return;
    }
    resplan_situation = !resplan_situation;
    if (resplan_situation) convert_result_table_to_daily();
    else convert_result_table_to_ave();
}

function convert_result_table_to_ave() {
    let res_table = document.querySelector(
        "body div.form.row div.result-surrounder div.result-table-surrounder table tbody"
    );
    res_table.innerHTML = "";
    let res = resplan.result_average;
    let refresh = resplan.projects;
    let title = document.querySelector(
        "body div.form.row div.result-surrounder div.result-table-surrounder h5 a"
    );
    title.innerHTML = "平均科研产出&nbsp;&nbsp;";
    res_table.appendChild(fill_table("物资消耗", fetch_doubloon(res.doubloon)));
    res_table.appendChild(fill_table("魔方消耗", res.cube.toFixed(2)));
    res_table.appendChild(fill_table("科研用时", res.time.toFixed(2) + "H"));
    res_table.appendChild(fill_table("金图纸产出", res.ssr_blp.toFixed(2)));
    res_table.appendChild(fill_table("彩图纸产出", res.ur_blp.toFixed(2)));
    res_table.appendChild(fill_table("彩装备产出", res.ur_equip.toFixed(2)));
    res_table.appendChild(
        fill_table("心智单元产出", res.cogn_chips.toFixed(2))
    );
    res_table.appendChild(
        fill_table("使用刷新概率", refresh.refresh_rate.toFixed(2))
    );
    res_table.appendChild(
        fill_table("刷新失败概率", refresh.refresh_fail.toFixed(2))
    );
}

function convert_result_table_to_daily() {
    let res_table = document.querySelector(
        "body div.form.row div.result-surrounder div.result-table-surrounder table tbody"
    );
    let res = resplan.result;
    res_table.innerHTML = "";
    let title = document.querySelector(
        "body div.form.row div.result-surrounder div.result-table-surrounder h5 a"
    );
    title.innerHTML = "每日科研产出&nbsp;&nbsp;";
    res_table.appendChild(fill_table("物资消耗", fetch_doubloon(res.doubloon)));
    res_table.appendChild(fill_table("魔方消耗", res.cube.toFixed(2)));
    res_table.appendChild(
        fill_table("科研次数", res.research_per_day.toFixed(2))
    );
    res_table.appendChild(
        fill_table(
            "上线间隔",
            ((24 / res.research_per_day) * 5).toFixed(2) + "H"
        )
    );
    res_table.appendChild(fill_table("金图纸产出", res.ssr_blp.toFixed(2)));
    res_table.appendChild(fill_table("彩图纸产出", res.ur_blp.toFixed(2)));
    res_table.appendChild(fill_table("彩装备产出", res.ur_equip.toFixed(2)));
    res_table.appendChild(
        fill_table("心智单元产出", res.cogn_chips.toFixed(2))
    );
    res_table.appendChild(
        fill_table("参考期望", res.cost_performance.toFixed(5))
    );
    res_table.appendChild(
        fill_table("参考刷新期望", res.cost_refresh_performance.toFixed(5))
    );
}

function fill_table(attr_name, attr_value) {
    let tr = document.createElement("tr");
    let tdan = document.createElement("td");
    tdan.innerHTML = attr_name;
    let tdav = document.createElement("td");
    tdav.innerHTML = attr_value;
    tr.appendChild(tdan);
    tr.appendChild(tdav);
    return tr;
}

function convert_predict_table() {
    if (!resplan || !restriction || !referenceValue) {
        alert("请先刷新数据呢");
        return;
    }
    if (predict_situation === 2) predict_situation = 0;
    else predict_situation += 1;
    // alert(predict_situation)
    if (predict_situation === 0) predict_situation_0();
    else if (predict_situation === 1) predict_situation_1();
    else predict_situation_2();
}

function predict_situation_0() {
    let predict_table = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder table tbody"
    );
    predict_table.innerHTML = "";
    let res = resplan.result;
    let title = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder h5 a"
    );
    title.innerHTML = "全年产出&nbsp;&nbsp;";
    predict_table.appendChild(
        fill_table("物资消耗", fetch_doubloon(res.doubloon * 365))
    );
    predict_table.appendChild(
        fill_table("魔方消耗", (res.cube * 365).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("科研总次数", (res.research_per_day * 365).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("金图纸产出", (res.ssr_blp * 365).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("彩图纸产出", (res.ur_blp * 365).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("彩装备产出", (res.ur_equip * 365).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("心智单元产出", (res.cogn_chips * 365).toFixed(2))
    );
}

function predict_situation_1() {
    let predict_table = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder table tbody"
    );
    predict_table.innerHTML = "";
    let res = resplan.result;
    let title = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder h5 a"
    );
    title.innerHTML = "科研船从零满破&nbsp;&nbsp;";
    let ssr_time = 343 * 3;
    if (restriction.fni_5_super_r === 3) ssr_time = 0;
    let ur_time = 513 * 2;
    if (restriction.fni_5_ultra_r === 2) ur_time = 0;
    let ftime = Math.max(ssr_time, ur_time);
    predict_table.appendChild(
        fill_table("金船满破", ssr_time.toFixed(2) + "天")
    );
    predict_table.appendChild(
        fill_table("彩船满破", ur_time.toFixed(2) + "天")
    );
    predict_table.appendChild(
        fill_table("合计满破用时", ftime.toFixed(2) + "天")
    );
    predict_table.appendChild(
        fill_table("物资消耗", fetch_doubloon(res.doubloon * ftime))
    );
    predict_table.appendChild(
        fill_table("魔方消耗", (res.cube * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("科研总次数", (res.research_per_day * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("彩装备产出", (res.ur_equip * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("心智单元产出", (res.cogn_chips * ftime).toFixed(2))
    );
}

function predict_situation_2() {
    let predict_table = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder table tbody"
    );
    predict_table.innerHTML = "";
    let res = resplan.result;
    let title = document.querySelector(
        "body div.form.row div.result-surrounder div.predict-table-surrounder h5 a"
    );
    title.innerHTML = "直到六期开始&nbsp;&nbsp;";
    let ftime = (END_DATE - new Date()) / 1000 / 3600 / 24;
    predict_table.appendChild(
        fill_table("物资消耗", fetch_doubloon(res.doubloon * ftime))
    );
    predict_table.appendChild(
        fill_table("魔方消耗", (res.cube * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("科研总次数", (res.research_per_day * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("金图纸产出", (res.ssr_blp * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("彩图纸产出", (res.ur_blp * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("彩装备产出", (res.ur_equip * ftime).toFixed(2))
    );
    predict_table.appendChild(
        fill_table("心智单元产出", (res.cogn_chips * ftime).toFixed(2))
    );
    predict_table.appendChild(fill_table("剩余时间", ftime.toFixed(2) + "天"));
}

function fetch_doubloon(d) {
    if (d >= 100000) return (d / 10000).toFixed(2) + "万";
    else return d.toFixed(2);
}

function convert_higher_level_option() {
    higher_level_option = !higher_level_option;
    if (higher_level_option) higher_level_option_on();
    else higher_level_option_off();
}

function higher_level_option_on() {
    let left = document.getElementById("rest-surrounder");
    let mid = document.getElementById("refer-surrounder");
    let right = document.getElementById("result-surrounder");
    let btn = document.getElementById("high_level_option");
    btn.innerHTML = "高级选项-已开启";
    btn.classList.add("btn-primary");
    btn.classList.remove("btn-outline-primary");
    left.classList.remove("col-xl-4");
    left.classList.add("col-xl-3");
    left.classList.add("col-md-6");
    mid.classList.remove("d-none");
    right.classList.remove("col-xl-8");
    right.classList.add("col-xl-7");
}

function higher_level_option_off() {
    let left = document.getElementById("rest-surrounder");
    let mid = document.getElementById("refer-surrounder");
    let right = document.getElementById("result-surrounder");
    let btn = document.getElementById("high_level_option");
    btn.innerHTML = "高级选项-已关闭";
    btn.classList.remove("btn-primary");
    btn.classList.add("btn-outline-primary");
    left.classList.add("col-xl-4");
    left.classList.remove("col-xl-3");
    left.classList.remove("col-md-6");
    mid.classList.add("d-none");
    right.classList.add("col-xl-8");
    right.classList.remove("col-xl-7");
}
