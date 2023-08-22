{
{% for data in dataset %}
m.push(ResearchData {
    research_id: {{data.research_id}},
    proj_type: cvt_str2u8("{{data.proj_type}}"),
    time: {{data.time}}_f64,
    rate: {{data.rate}}_f64,
    doubloon: {{data.doubloon}},
    cube: {{data.cube}},
    super_rare_blp: {{data.super_rare_blp}}_f64,
    ultra_rare_blp: {{data.ultra_rare_blp}}_f64,
    ultra_rare_equip: {{data.ultra_rare_equip}}_f64,
    cognitive_chips: {{data.cognitive_chips}}_f64
});
{% endfor %}
}