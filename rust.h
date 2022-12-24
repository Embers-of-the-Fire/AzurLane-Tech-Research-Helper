#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RefProjContent RefProjContent;

typedef struct Restriction {
  double doubloon_ratio;
  double cube_ratio;
  double cong_chips_ratio;
  double time_ratio;
  double ultra_blp_ratio;
  double ultra_equip_ratio;
  int8_t fni_5_super_r;
  int8_t fni_5_ultra_r;
  int8_t fni_f;
  bool do_data_collection;
  bool do_research_assignment;
} Restriction;

typedef struct ReferenceValue {
  double doubloon;
  double cube;
  double time_of_an_hour;
  double super_rare;
  double ultra_rare;
  double ultra_equip;
  double cong_chips;
  double time_ratio;
} ReferenceValue;

typedef struct ResearchResult {
  double doubloon;
  double cube;
  double time;
  double ssr_blp;
  double ur_blp;
  double ur_equip;
  double cogn_chips;
  double cost_performance;
  double cost_refresh_performance;
} ResearchResult;

typedef struct ResultPerDay {
  double doubloon;
  double cube;
  double research_per_day;
  double ssr_blp;
  double ur_blp;
  double ur_equip;
  double cogn_chips;
  double cost_performance;
  double cost_refresh_performance;
} ResultPerDay;

typedef struct VecRep_RefProjContent {
  const struct RefProjContent *ptr;
  uintptr_t size;
} VecRep_RefProjContent;

typedef struct RefreshProjectsRepV {
  struct VecRep_RefProjContent data;
  double average_time;
  double cost_performance;
  double cost_refresh_performance;
  double research_time_per_day;
  double total_select_rate;
  double refresh_rate;
  double refresh_fail;
} RefreshProjectsRepV;

typedef struct ResultPlanRepV {
  struct ResearchResult result_average;
  struct ResultPerDay result;
  struct RefreshProjectsRepV projects;
} ResultPlanRepV;

struct Restriction predef_restriction(void);

struct ReferenceValue predef_reference_value(void);

struct Restriction build_restriction(double doubloon_ratio,
                                     double cube_ratio,
                                     double cong_chips_ratio,
                                     double time_ratio,
                                     double ultra_blp_ratio,
                                     double ultra_equip_ratio,
                                     int8_t fni_5_super_r,
                                     int8_t fni_5_ultra_r,
                                     int8_t fni_f,
                                     bool do_data_collection,
                                     bool do_research_assignment);

struct ReferenceValue build_reference_value(double doubloon,
                                            double cube,
                                            double time_of_an_hour,
                                            double super_rare,
                                            double ultra_rare,
                                            double ultra_equip,
                                            double cong_chips,
                                            double time_ratio);

struct ResultPlanRepV calc(struct Restriction *rest,
                           struct ReferenceValue *raw_ref,
                           double refer_v,
                           int8_t limit);
