export type Restriction = {
    doubloon_ratio: number;
    cube_ratio: number;
    cong_chips_ratio: number;
    time_ratio: number;
    ultra_blp_ratio: number;
    ultra_equip_ratio: number;
    fni_5_super_r: number;
    fni_5_ultra_r: number;
    fni_f: number;
    do_data_collection: boolean;
    do_research_assignment: boolean;
};

export type ReferenceValue = {
    doubloon: number;
    cube: number;
    time_of_an_hour: number;
    super_rare: number;
    ultra_rare: number;
    ultra_equip: number;
    cong_chips: number;
    time_ratio: number;
};

export type ResearchResult = {
    doubloon: number;
    cube: number;
    time: number;
    ssr_blp: number;
    ur_blp: number;
    ur_equip: number;
    cogn_chips: number;
    cost_performance: number;
    cost_refresh_performance: number;
};

export type ResultPerDay = {
    doubloon: number;
    cube: number;
    research_per_day: number;
    ssr_blp: number;
    ur_blp: number;
    ur_equip: number;
    cogn_chips: number;
    cost_performance: number;
    cost_refresh_performance: number;
};

export type ResearchData = {
    research_id: number;
    proj_type: number;
    time: number;
    rate: number;
    doubloon: number;
    cube: number;
    super_rare_blp: number;
    ultra_rare_blp: number;
    ultra_rare_equip: number;
    cognitive_chips: number;
};

export type ActualResearch = {
    data: ResearchData;
    single_income: number;
    cost: number;
    refer_income: number;
    tap_income: number;
};

export type RefreshProject = {
    rate: number;
    r1: number; // 刷出率1
    r2: number; // 刷出率2
    false_: number; // 无
    true_: number; // 有
    select_rate: number; // 选取率
    select: boolean; //是否选取
    sr1: number; // 选取率1
    sr2: number; // 选取率2
    sr3: number; // 选取率3
    actual_select: number; // 实际选取
};

export type RefProjContent = {
    data: ActualResearch;
    ref_proj: RefreshProject;
};

export type RefreshProjects = {
    data: RefProjContent[];
    average_time: number;
    cost_performance: number; // '刷新情况表'!G112
    cost_refresh_performance: number; // '刷新情况表'!J112
    research_time_per_day: number; // '刷新情况表'!number
    total_select_rate: number; // '刷新情况表'!K31
    refresh_rate: number; // '刷新情况表'!K33
    refresh_fail: number; // '刷新情况表'!K34
};

export type ResultPlan = {
    result_average: ResearchResult;
    result: ResultPerDay;
    projects: RefreshProjects;
};

export function predef_restriction(): Restriction;

export function predef_reference_value(): ReferenceValue;

export function build_restriction(
    doubloon_ratio: number,
    cube_ratio: number,
    cong_chips_ratio: number,
    time_ratio: number,
    ultra_blp_ratio: number,
    ultra_equip_ratio: number,
    fni_5_super_r: number,
    fni_5_ultra_r: number,
    fni_f: number,
    do_data_collection: boolean,
    do_research_assignment: boolean
): Restriction;

export function build_reference_value(
    doubloon: number,
    cube: number,
    time_of_an_hour: number,
    super_rare: number,
    ultra_rare: number,
    ultra_equip: number,
    cong_chips: number,
    time_ratio: number
): ReferenceValue;

export function calc(
    rest: Restriction,
    raw_ref: ReferenceValue,
    refer_v: number,
    limit: number
): ResultPlan;
