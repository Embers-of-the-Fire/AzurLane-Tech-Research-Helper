'use strict';
/* eslint-disable */

const NAME_LIST = ["魔方解析1h", "魔方解析2h", "魔方解析4h", "魔方解析0.5h", "舰装解析1h", "舰装解析2h", "舰装解析4h", "舰装解析0.5h", "金船定向2.5h", "金船定向5h", "金船定向8h", "金船定向0.5h", "彩船定向2.5h", "彩船定向5h", "彩船定向8h", "彩船定向0.5h", "资金募集1.5h", "资金募集2.5h", "资金募集4h", "蓝试验品募集2h", "紫试验品募集2h", "紫数据收集4h", "金数据收集4h", "基础研究6h", "基础研究8h", "基础研究12h", "研究委托3h", "研究委托4h", "研究委托6h"];
const TRUE_IMG = '&#10004;';
const FALSE_IMG = '&#10006;';
let resplan_situation = true; // true for daily and false for average
let higher_level_option = false; // true for on and false for off
let predict_situation = 0; // 0 for "全年产出", 1 for "科研船从零满破", 2 for "直到六期开始"
let restriction, resplan, referenceValue;
const END_DATE = new Date('2023/7/13  9:00:00');

import init, {build_reference_value, build_restriction, calc} from './azurlane_tech_research.js';
// initSync();
// import initSync, { predef_reference_value, predef_restriction, calc, build_restriction, build_reference_value } from "azurlane-tech-research";
// azurlane.initSync();


// document.addEventListener('load', function() {
//     alert('test');
//     document.getElementById("refresh").addEventListener('click', submit);
// });

init();

export function submit() {
    init();
    let formRes = document.getElementById('RestrictionForm');
    let formRef = document.getElementById('ReferenceForm');
    formRes.classList.add('was-validated');
    formRef.classList.add('was-validated');
    if (formRes.checkValidity() !== true || formRef.checkValidity() !== true) return;
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
    )
    referenceValue = build_reference_value(
        formRef.doubloon_refer.value,
        formRef.cube_refer.value,
        formRef.time_refer.value,
        formRef.ssr_refer.value,
        formRef.ur_refer.value,
        formRef.ur_equip_refer.value,
        formRef.cogn_chips_refer.value,
        formRef.time_refer.value * 72
    )
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
        } catch(err) {
            alert('出错了！\n错误内容：' + err + '\n请修改入参后重试，如果仍然错误，请向作者报告');
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
        if (lim == 50) alert("迭代次数超过预设次数50次，直接保留最后一次结果，请尝试修改迭代初始值，或将最后一次结果的参考值作为初始值以继续迭代");
    };
    // alert(resplan.result.cost_performance)
    let table_resproject = document.querySelector("body > div.form.row > div.result-surrounder > div.tech-table-surrounder > table > tbody");
    table_resproject.innerHTML = '';
    let pl = resplan.projects.data;
    let nameid, name;
    let newtr, newtdindex, newtdname, newtdselect;
    for (var i=0;i<pl.length;i++) {
        nameid = pl[i].data.data.research_id;
        name = NAME_LIST[nameid];
        newtdindex = document.createElement("td");
        newtdindex.innerHTML = i+1;
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
    };
    convert_result_table_to_daily();
    predict_situation_0();
}

function string_formatter(str) {
	var args = arguments, re = new RegExp("%([1-" + args.length + "])", "g");
	return String(str).replace(
		re,
		function(_, $2) {
			return args[$2];
		}
	);
};

export function convert_result_table() {
    if (!resplan || !restriction || !referenceValue) { alert("请先刷新数据呢"); return; };
    resplan_situation = !resplan_situation;
    if (resplan_situation) convert_result_table_to_daily();
    else convert_result_table_to_ave();
}

function convert_result_table_to_ave() {
    let res_table = document.querySelector("body > div.form.row > div.result-surrounder > div.result-table-surrounder > table > tbody");
    res_table.innerHTML = '';
    let res = resplan.result_average;
    let refresh = resplan.projects;
    let title = document.querySelector("body > div.form.row > div.result-surrounder > div.result-table-surrounder > h5 > a")
    title.innerHTML = "平均科研产出&nbsp;&nbsp;";
    res_table.appendChild(fill_table('物资消耗', fetch_doubloon(res.doubloon)));
    res_table.appendChild(fill_table('魔方消耗', res.cube.toFixed(2)));
    res_table.appendChild(fill_table('科研用时', res.time.toFixed(2) + 'H'));
    res_table.appendChild(fill_table('金图纸产出', res.ssr_blp.toFixed(2)));
    res_table.appendChild(fill_table('彩图纸产出', res.ur_blp.toFixed(2)));
    res_table.appendChild(fill_table('彩装备产出', res.ur_equip.toFixed(2)));
    res_table.appendChild(fill_table('心智单元产出', res.cogn_chips.toFixed(2)));
    res_table.appendChild(fill_table('使用刷新概率', refresh.refresh_rate.toFixed(2)));
    res_table.appendChild(fill_table('刷新失败概率', refresh.refresh_fail.toFixed(2)));
}

function convert_result_table_to_daily() {
    let res_table = document.querySelector("body > div.form.row > div.result-surrounder > div.result-table-surrounder > table > tbody");
    let res = resplan.result;
    res_table.innerHTML = '';
    let title = document.querySelector("body > div.form.row > div.result-surrounder > div.result-table-surrounder > h5 > a")
    title.innerHTML = "每日科研产出&nbsp;&nbsp;";
    res_table.appendChild(fill_table('物资消耗', fetch_doubloon(res.doubloon)));
    res_table.appendChild(fill_table('魔方消耗', res.cube.toFixed(2)));
    res_table.appendChild(fill_table('科研次数', res.research_per_day.toFixed(2)));
    res_table.appendChild(fill_table('上线间隔', (24 / res.research_per_day * 5).toFixed(2) + 'H'));
    res_table.appendChild(fill_table('金图纸产出', res.ssr_blp.toFixed(2)));
    res_table.appendChild(fill_table('彩图纸产出', res.ur_blp.toFixed(2)));
    res_table.appendChild(fill_table('彩装备产出', res.ur_equip.toFixed(2)));
    res_table.appendChild(fill_table('心智单元产出', res.cogn_chips.toFixed(2)));
    res_table.appendChild(fill_table('参考期望', res.cost_performance.toFixed(5)));
    res_table.appendChild(fill_table('参考刷新期望', res.cost_refresh_performance.toFixed(5)));
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

export function convert_predict_table() {
    if (!resplan || !restriction || !referenceValue) { alert("请先刷新数据呢"); return; };
    if (predict_situation === 2) predict_situation = 0;
    else predict_situation += 1;
    // alert(predict_situation)
    if (predict_situation === 0) predict_situation_0();
    else if (predict_situation === 1) predict_situation_1();
    else predict_situation_2();
}

function predict_situation_0() {
    let predict_table = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > table > tbody");
    predict_table.innerHTML = '';
    let res = resplan.result;
    let title = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > h5 > a");
    title.innerHTML = "全年产出&nbsp;&nbsp;";
    predict_table.appendChild(fill_table('物资消耗', fetch_doubloon(res.doubloon * 365)));
    predict_table.appendChild(fill_table('魔方消耗', (res.cube * 365).toFixed(2)));
    predict_table.appendChild(fill_table('科研总次数', (res.research_per_day * 365).toFixed(2)));
    predict_table.appendChild(fill_table('金图纸产出', (res.ssr_blp * 365).toFixed(2)));
    predict_table.appendChild(fill_table('彩图纸产出', (res.ur_blp * 365).toFixed(2)));
    predict_table.appendChild(fill_table('彩装备产出', (res.ur_equip * 365).toFixed(2)));
    predict_table.appendChild(fill_table('心智单元产出', (res.cogn_chips * 365).toFixed(2)));
}

function predict_situation_1() {
    let predict_table = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > table > tbody");
    predict_table.innerHTML = '';
    let res = resplan.result;
    let title = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > h5 > a");
    title.innerHTML = "科研船从零满破&nbsp;&nbsp;";
    let ssr_time = 343 * 3;
    if (restriction.fni_5_super_r === 3) ssr_time = 0;
    let ur_time = 513 * 2;
    if (restriction.fni_5_ultra_r === 2) ur_time = 0;
    let ftime = Math.max(ssr_time, ur_time);
    predict_table.appendChild(fill_table('金船满破', ssr_time.toFixed(2) + '天'));
    predict_table.appendChild(fill_table('彩船满破', ur_time.toFixed(2) + '天'));
    predict_table.appendChild(fill_table('合计满破用时', ftime.toFixed(2) + '天'));
    predict_table.appendChild(fill_table('物资消耗', fetch_doubloon(res.doubloon * ftime)));
    predict_table.appendChild(fill_table('魔方消耗', (res.cube * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('科研总次数', (res.research_per_day * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('彩装备产出', (res.ur_equip * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('心智单元产出', (res.cogn_chips * ftime).toFixed(2)));
}

function predict_situation_2() {
    let predict_table = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > table > tbody");
    predict_table.innerHTML = '';
    let res = resplan.result;
    let title = document.querySelector("body > div.form.row > div.result-surrounder > div.predict-table-surrounder > h5 > a");
    title.innerHTML = "直到六期开始&nbsp;&nbsp;";
    let ftime = (END_DATE - new Date()) / 1000 / 3600 / 24;
    predict_table.appendChild(fill_table('物资消耗', fetch_doubloon(res.doubloon * ftime)));
    predict_table.appendChild(fill_table('魔方消耗', (res.cube * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('科研总次数', (res.research_per_day * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('金图纸产出', (res.ssr_blp * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('彩图纸产出', (res.ur_blp * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('彩装备产出', (res.ur_equip * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('心智单元产出', (res.cogn_chips * ftime).toFixed(2)));
    predict_table.appendChild(fill_table('剩余时间', ftime.toFixed(2) + '天'));
}

function fetch_doubloon(d) {
    if (d >= 100000) return (d / 10000).toFixed(2) + '万';
    else return d.toFixed(2);
}

export function convert_higher_level_option() {
    higher_level_option = !higher_level_option;
    if (higher_level_option) higher_level_option_on();
    else higher_level_option_off();
}

function higher_level_option_on() {
    let left = document.getElementById('rest-surrounder');
    let mid = document.getElementById('refer-surrounder');
    let right = document.getElementById('result-surrounder');
    let btn = document.getElementById('high_level_option');
    btn.innerHTML = '高级选项-已开启';
    btn.classList.add('btn-primary');
    btn.classList.remove('btn-outline-primary');
    left.classList.remove('col-xl-4');
    left.classList.add('col-xl-3');
    left.classList.add('col-md-6');
    mid.classList.remove('d-none');
    right.classList.remove('col-xl-8');
    right.classList.add('col-xl-7');
}

function higher_level_option_off() {
    let left = document.getElementById('rest-surrounder');
    let mid = document.getElementById('refer-surrounder');
    let right = document.getElementById('result-surrounder');
    let btn = document.getElementById('high_level_option');
    btn.innerHTML = '高级选项-已关闭';
    btn.classList.remove('btn-primary');
    btn.classList.add('btn-outline-primary');
    left.classList.add('col-xl-4');
    left.classList.remove('col-xl-3');
    left.classList.remove('col-md-6');
    mid.classList.add('d-none');
    right.classList.add('col-xl-8');
    right.classList.remove('col-xl-7');
}
