# DSH 各行业插件 / MCP / Skill 组合包大全（补充篇）

> 版本：v1.1.0（补充篇）  
> 日期：2026-09-04  
> 说明：本文档为《DSH 各行业插件/MCP/Skill 组合包大全》的补充篇，新增 15 个行业、45 个组合包。与主文档配合使用，覆盖更广泛的行业场景。

---

## 目录

- [16. 汽车行业](#16-汽车行业)
- [17. 能源与公用事业](#17-能源与公用事业)
- [18. 体育与健身](#18-体育与健身)
- [19. 文化艺术与文博](#19-文化艺术与文博)
- [20. 游戏与电竞](#20-游戏与电竞)
- [21. 环保与碳中和](#21-环保与碳中和)
- [22. 宠物行业](#22-宠物行业)
- [23. 母婴与育儿](#23-母婴与育儿)
- [24. 养老与康养](#24-养老与康养)
- [25. 咨询与专业服务](#25-咨询与专业服务)
- [26. 会计审计与税务](#26-会计审计与税务)
- [27. 翻译与本地化](#27-翻译与本地化)
- [28. 婚庆与礼仪](#28-婚庆与礼仪)
- [29. 美容美业](#29-美容美业)
- [30. 殡葬与生命服务](#30-殡葬与生命服务)

---

## 16. 汽车行业

### 组合包 AUTO-01：汽车销售与 4S 店运营包

**适用场景：** 汽车 4S 店、汽车经销商、二手车商

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-auto-sales` | 汽车销售管理，客户跟进，试驾预约，报价单生成 |
| 插件 | `dsh-plugin-vehicle-inventory` | 车辆库存管理，在库车辆，VIN 管理，进销存跟踪 |
| 插件 | `dsh-plugin-after-sales` | 售后服务管理，维修预约，保养提醒，客户回访 |
| MCP | `automotive-database-mcp` | 汽车数据库接口，车型参数，配置对比，价格查询 |
| MCP | `dealer-management-mcp` | 经销商管理系统 DMS 接口 |
| MCP | `used-car-valuation-mcp` | 二手车估值接口，车况评估，残值计算 |
| Skill | `customer-follow-up-auto` | 汽车销售客户跟进，需求分析，报价方案，谈判策略，成交促进 |
| Skill | `vehicle-comparison` | 车型对比分析，参数对比，优劣势分析，推荐理由，客户匹配 |
| Skill | `after-sales-service` | 售后服务流程，维修接待，估价说明，保养建议，增值服务推荐，客户满意度 |

---

### 组合包 AUTO-02：汽车维修与保养包

**适用场景：** 汽车维修厂、快修店、保养连锁、轮胎店

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-repair-management` | 维修管理，工单创建，故障诊断，维修记录 |
| 插件 | `dsh-plugin-parts-management` | 配件管理，库存查询，采购申请，出入库管理 |
| 插件 | `dsh-plugin-vehicle-diagnosis` | 车辆诊断辅助，故障码解析，维修方案，工时估算 |
| MCP | `obd2-diagnostic-mcp` | OBD-II 诊断接口，故障码读取，实时数据，车辆状态 |
| MCP | `parts-catalog-mcp` | 配件目录接口，OE 号查询，配件适配，价格库存 |
| MCP | `repair-manual-mcp` | 维修手册接口，拆装步骤，扭矩参数，电路图 |
| Skill | `fault-diagnosis` | 故障诊断辅助，症状分析，故障码解读，排查步骤，维修方案，配件清单 |
| Skill | `repair-quote` | 维修报价生成，工时计算，配件价格，管理费，利润分析，报价单 |
| Skill | `maintenance-plan` | 保养计划制定，保养项目，周期建议，油品选择，易损件更换，费用预估 |

---

### 组合包 AUTO-03：汽车制造与供应链包

**适用场景：** 汽车制造厂、零部件供应商、汽车电子企业

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-auto-manufacturing` | 汽车制造管理，生产计划，装配线管理，质量追溯 |
| 插件 | `dsh-plugin-auto-parts-supply` | 零部件供应链管理，供应商管理，JIT 配送，库存协同 |
| 插件 | `dsh-plugin-automotive-qa` | 汽车质量管理，IATF 16949，APQP/PPAP，8D 报告 |
| MCP | `mes-automotive-mcp` | 汽车行业 MES 系统接口 |
| MCP | `plm-mcp` | PLM 产品生命周期管理接口，BOM 管理，图纸管理 |
| MCP | `supply-chain-auto-mcp` | 汽车供应链接口，订单协同，发货通知，在途跟踪 |
| Skill | `production-planning-auto` | 汽车生产计划编制，产能平衡，物料需求，排程优化，在制品管理 |
| Skill | `quality-control-auto` | 汽车质量控制，检验标准，抽样方案，缺陷分类，SPC 统计，整改跟踪 |
| Skill | `supplier-quality-management` | 供应商质量管理，准入审核，绩效评估，质量协议，问题整改，年度评审 |

---

## 17. 能源与公用事业

### 组合包 ENERGY-01：电力运营与电网包

**适用场景：** 电力公司、电网企业、供电所、新能源电站

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-power-grid` | 电网运营管理，调度运行，设备监控，故障处理 |
| 插件 | `dsh-plugin-meter-reading` | 抄表管理，电费核算，账单生成，欠费催缴 |
| 插件 | `dsh-plugin-power-customer` | 电力客户服务，报装接电，故障报修，咨询投诉 |
| MCP | `scada-mcp` | SCADA 数据采集与监控系统接口，实时数据，告警信息 |
| MCP | `smart-meter-mcp` | 智能电表接口，用电数据，远程抄表，负荷控制 |
| MCP | `gis-power-mcp` | 电力 GIS 接口，电网拓扑，设备定位，线路管理 |
| Skill | `grid-dispatch` | 电网调度运行，负荷预测，发电计划，潮流计算，安全校核，应急处置 |
| Skill | `power-billing` | 电费核算管理，电价政策，阶梯电价，力调电费，账单生成，对账核对 |
| Skill | `power-outage-management` | 停电管理，计划停电，故障停电，影响分析，通知发布，复电跟踪，满意度调查 |

---

### 组合包 ENERGY-02：新能源与可再生能源包

**适用场景：** 光伏电站、风电场、储能电站、新能源企业

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-renewable-plant` | 新能源电站管理，发电监控，设备运维，性能分析 |
| 插件 | `dsh-plugin-energy-storage` | 储能系统管理，充放电调度，SOC 管理，寿命评估 |
| 插件 | `dsh-plugin-carbon-accounting` | 碳核算管理，碳排放计算，碳足迹追踪，碳减排量 |
| MCP | `pv-monitoring-mcp` | 光伏监控接口，逆变器数据，发电量，组件状态 |
| MCP | `wind-scada-mcp` | 风电 SCADA 接口，风机数据，风速风向，发电量 |
| MCP | `weather-forecast-mcp` | 气象预报接口，辐照度，风速，温度，发电预测 |
| MCP | `carbon-market-mcp` | 碳市场接口，碳价查询，碳交易，CCER |
| Skill | `renewable-performance` | 新能源电站性能分析，PR 计算，可利用率，故障统计，损失分析，优化建议 |
| Skill | `energy-storage-dispatch` | 储能调度优化，充放电策略，峰谷套利，需求响应，SOC 管理，寿命优化 |
| Skill | `carbon-footprint-calculation` | 碳足迹计算，排放因子，活动数据，范围一二三，碳减排量，碳报告生成 |

---

### 组合包 ENERGY-03：石油天然气与公用事业包

**适用场景：** 石油公司、天然气公司、水务公司、燃气公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-oil-gas-operations` | 油气运营管理，生产运行，管道输送，场站管理 |
| 插件 | `dsh-plugin-water-utility` | 水务管理，供水调度，水质监测，管网管理 |
| 插件 | `dsh-plugin-gas-utility` | 燃气管理，输配调度，安全监控，客户服务 |
| MCP | `pipeline-scada-mcp` | 管道 SCADA 接口，压力流量，温度，泄漏检测 |
| MCP | `water-quality-mcp` | 水质监测接口，pH，浊度，余氯，微生物 |
| MCP | `gas-leak-detection-mcp` | 燃气泄漏检测接口，浓度监测，报警定位 |
| Skill | `oil-gas-production` | 油气生产管理，产量分析，井口管理，注水注气，增产措施，成本核算 |
| Skill | `water-supply-management` | 供水运营管理，水源调度，水厂运行，管网压力，漏损控制，水质保障 |
| Skill | `gas-safety-management` | 燃气安全管理，隐患排查，泄漏应急，入户安检，用户宣传，事故预案 |

---

## 18. 体育与健身

### 组合包 SPORT-01：健身俱乐部与私教包

**适用场景：** 健身房、健身工作室、私教团队、瑜伽馆

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-gym-management` | 健身房管理，会员管理，课程排期，场地预约 |
| 插件 | `dsh-plugin-personal-training` | 私教管理，训练计划，动作库，训练记录，效果跟踪 |
| 插件 | `dsh-plugin-fitness-assessment` | 体测管理，体成分分析，体能测试，健康评估 |
| MCP | `fitness-tracker-mcp` | 健身追踪设备接口（Apple Health/Google Fit/华为运动健康） |
| MCP | `exercise-database-mcp` | 动作库数据库接口，动作图解，肌肉群，难度等级 |
| MCP | `nutrition-database-mcp` | 营养数据库接口，食物热量，营养素，饮食记录 |
| Skill | `training-plan-design` | 训练计划设计，目标设定，周期划分，动作选择，组数次数，进阶调整 |
| Skill | `fitness-assessment-report` | 体测报告生成，体成分分析，体态评估，体能评级，风险提示，改进建议 |
| Skill | `member-retention-fitness` | 会员留存运营，到店提醒，训练激励，效果展示，续费沟通，转介绍激励 |

---

### 组合包 SPORT-02：体育赛事与运营包

**适用场景：** 体育赛事公司、赛事运营方、体育协会、俱乐部

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-event-planning` | 赛事策划，方案设计，预算编制，流程安排 |
| 插件 | `dsh-plugin-registration` | 报名管理，选手注册，报名费，分组编排 |
| 插件 | `dsh-plugin-scoring-timing` | 计时计分，成绩记录，排名计算，证书生成 |
| MCP | `timing-system-mcp` | 计时系统接口，芯片计时，终点摄像，分段成绩 |
| MCP | `sports-data-mcp` | 体育数据接口，选手数据，历史成绩，统计分析 |
| MCP | `venue-booking-mcp` | 场馆预订接口，场地查询，预约管理，费用结算 |
| Skill | `event-planning-sports` | 赛事策划方案，赛事定位，赛程设计，场地规划，人员配置，预算明细，风险预案 |
| Skill | `competition-management` | 竞赛组织管理，报名审核，分组抽签，赛程编排，裁判安排，成绩处理，颁奖组织 |
| Skill | `sports-marketing` | 体育赛事营销，赞助招商，媒体宣传，票务销售，观众体验，品牌曝光，商业开发 |

---

### 组合包 SPORT-03：运动康复与运动医学包

**适用场景：** 运动康复中心、康复诊所、运动队医、物理治疗师

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-sports-rehab` | 运动康复管理，评估诊断，康复计划，治疗记录 |
| 插件 | `dsh-plugin-injury-management` | 损伤管理，损伤评估，恢复跟踪，回归运动 |
| 插件 | `dsh-plugin-physiotherapy` | 物理治疗管理，治疗方案，手法记录，设备使用 |
| MCP | `rehab-equipment-mcp` | 康复设备接口，治疗设备数据，训练参数，效果记录 |
| MCP | `medical-imaging-mcp` | 医学影像接口，X光/MRI/CT，影像查看，报告读取 |
| MCP | `pain-assessment-mcp` | 疼痛评估接口，VAS 评分，疼痛地图，功能评分 |
| Skill | `injury-assessment` | 运动损伤评估，病史采集，体格检查，特殊试验，影像学解读，诊断意见 |
| Skill | `rehabilitation-program` | 康复方案制定，分期目标，治疗手段，训练动作，进阶标准，回归运动评估 |
| Skill | `return-to-sport` | 回归运动决策，功能测试，心理评估，运动专项测试，风险评估，逐步回归计划 |

---

## 19. 文化艺术与文博

### 组合包 CULTURE-01：博物馆与文博包

**适用场景：** 博物馆、纪念馆、美术馆、文博机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-collection-management` | 藏品管理，藏品登记，编目著录，库房管理 |
| 插件 | `dsh-plugin-exhibition` | 展览管理，策展方案，展品组织，展陈设计 |
| 插件 | `dsh-plugin-museum-education` | 社教活动，教育项目，导览讲解，研学课程 |
| MCP | `collection-database-mcp` | 藏品数据库接口，文物信息，图片，检索查询 |
| MCP | `museum-ticketing-mcp` | 博物馆票务接口，预约参观，门票销售，客流统计 |
| MCP | `audio-guide-mcp` | 语音导览接口，讲解内容，定位讲解，多语言 |
| Skill | `artifact-research` | 文物研究，断代辨伪，价值评估，历史考证，学术论文，展览说明 |
| Skill | `exhibition-curating` | 展览策划，主题定位，叙事结构，展品选择，空间设计，互动体验，教育活动 |
| Skill | `museum-public-education` | 博物馆社教，课程设计，活动策划，导览词撰写，研学方案，志愿者培训 |

---

### 组合包 CULTURE-02：艺术创作与画廊包

**适用场景：** 艺术家、画廊、艺术机构、拍卖行

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-artist-studio` | 艺术家工作室管理，作品管理，创作记录，素材库 |
| 插件 | `dsh-plugin-gallery-management` | 画廊管理，展览策划，作品销售，艺术家代理 |
| 插件 | `dsh-plugin-art-auction` | 拍卖管理，拍品征集，图录制作，拍卖执行 |
| MCP | `art-market-mcp` | 艺术市场数据接口，成交价，艺术家指数，市场趋势 |
| MCP | `artwork-registry-mcp` | 艺术品登记接口，作品溯源，真伪鉴定，所有权 |
| MCP | `image-generation-mcp` | AI 图像生成接口，创作辅助，风格迁移，概念图 |
| Skill | `art-creation-assistance` | 艺术创作辅助，灵感搜集，概念发展，构图建议，色彩方案，技法探索，批评反思 |
| Skill | `artwork-pricing` | 艺术品定价，市场分析，可比成交，艺术家定位，尺寸媒介，稀缺性，增值预期 |
| Skill | `exhibition-installation` | 展览布展，空间规划，作品排布，灯光设计，标签制作，开幕活动，媒体宣传 |

---

### 组合包 CULTURE-03：演出与演艺包

**适用场景：** 演出公司、剧院、剧团、演艺经纪

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-production-management` | 剧目制作管理，剧本管理，排练计划，制作预算 |
| 插件 | `dsh-plugin-theater-operations` | 剧院运营管理，演出排期，票务销售，场务管理 |
| 插件 | `dsh-plugin-performance-marketing` | 演出营销，宣传推广，媒体合作，观众运营 |
| MCP | `ticketing-system-mcp` | 票务系统接口，选座购票，票房统计，退票管理 |
| MCP | `script-database-mcp` | 剧本数据库接口，剧目信息，演职员，演出资料 |
| MCP | `venue-management-mcp` | 场馆管理接口，场地信息，设备清单，档期管理 |
| Skill | `script-analysis` | 剧本分析，主题解读，人物分析，结构梳理，舞台提示，导演阐述，排练计划 |
| Skill | `production-budget` | 制作预算编制，人员费用，场地设备，宣传营销，不可预见费，成本控制，结算审计 |
| Skill | `performance-marketing-plan` | 演出营销方案，目标受众，卖点提炼，渠道选择，内容策划，预售策略，口碑运营 |

---

## 20. 游戏与电竞

### 组合包 GAME-01：游戏开发与设计包

**适用场景：** 游戏开发团队、独立游戏开发者、游戏设计工作室

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-game-design` | 游戏设计，GDD 撰写，系统设计，数值设计，关卡设计 |
| 插件 | `dsh-plugin-game-development` | 游戏开发，代码管理，版本控制，Bug 跟踪 |
| 插件 | `dsh-plugin-game-asset` | 游戏资源管理，美术资源，音频资源，动画资源 |
| MCP | `unity-mcp` | Unity 引擎接口，场景操作，对象管理，脚本执行 |
| MCP | `unreal-mcp` | Unreal Engine 接口，蓝图操作，关卡编辑，资产管理 |
| MCP | `git-game-mcp` | Git 版本控制接口，代码仓库，分支管理，代码审查 |
| Skill | `gdd-writing` | 游戏设计文档撰写，核心玩法，系统设计，数值框架，经济系统，成长曲线，UI/UX |
| Skill | `game-balancing` | 游戏数值平衡，角色数值，装备数值，经济系统，难度曲线，PVP 平衡，数据驱动调整 |
| Skill | `level-design` | 关卡设计，关卡流程，空间布局，敌人配置，谜题设计，难度递进，玩家体验曲线 |

---

### 组合包 GAME-02：电竞俱乐部与赛事包

**适用场景：** 电竞俱乐部、电竞战队、电竞赛事方、电竞场馆

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-esports-team` | 电竞战队管理，选手管理，训练计划，比赛分析 |
| 插件 | `dsh-plugin-esports-event` | 电竞赛事管理，赛事策划，赛程编排，直播制作 |
| 插件 | `dsh-plugin-esports-venue` | 电竞场馆管理，场地运营，设备维护，赛事承办 |
| MCP | `game-data-mcp` | 游戏数据接口，比赛数据，选手数据，战术分析 |
| MCP | `streaming-platform-mcp` | 直播平台接口（斗鱼/虎牙/B站/Twitch），直播管理，弹幕互动 |
| MCP | `tournament-platform-mcp` | 赛事平台接口，报名管理，赛程管理，成绩统计 |
| Skill | `esports-training` | 电竞训练计划，战术训练，个人技术，团队配合，复盘分析，心理训练，体能管理 |
| Skill | `match-analysis` | 比赛复盘分析，数据统计，战术解读，失误分析，对手研究，改进方案，训练重点 |
| Skill | `esports-event-production` | 电竞赛事制作，赛事策划，赛程设计，OB 导播，解说安排，舞美灯光，互动环节 |

---

### 组合包 GAME-03：游戏运营与社区包

**适用场景：** 游戏运营团队、社区运营、玩家社区、游戏发行

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-game-operations` | 游戏运营管理，活动策划，版本更新，数据监控 |
| 插件 | `dsh-plugin-game-community` | 游戏社区管理，玩家互动，内容审核，社区活动 |
| 插件 | `dsh-plugin-game-customer-service` | 游戏客服，工单处理，BUG 反馈，玩家咨询 |
| MCP | `game-analytics-mcp` | 游戏数据分析接口，DAU/MAU，留存率，付费率，LTV |
| MCP | `discord-mcp` | Discord 社区接口，频道管理，消息发送，机器人 |
| MCP | `game-cs-mcp` | 游戏客服系统接口，工单管理，知识库，玩家信息 |
| Skill | `game-event-planning` | 游戏活动策划，节日活动，版本活动，付费活动，社交活动，奖励设计，数据预估 |
| Skill | `game-data-analysis` | 游戏数据分析，用户行为，留存分析，付费分析，流失预警，AB 测试，优化建议 |
| Skill | `community-operations-game` | 游戏社区运营，内容生态，KOL 培养，话题运营，活动策划，危机公关，社区氛围 |

---

## 21. 环保与碳中和

### 组合包 ENV-01：环境监测与治理包

**适用场景：** 环保公司、环境监测站、污染治理企业、环评机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-environmental-monitoring` | 环境监测管理，监测计划，数据采集，报告生成 |
| 插件 | `dsh-plugin-pollution-control` | 污染治理管理，治理方案，设备运行，效果评估 |
| 插件 | `dsh-plugin-eia` | 环境影响评价，报告编制，工程分析，环保措施 |
| MCP | `air-quality-mcp` | 空气质量监测接口，PM2.5/PM10，SO2/NOx/O3，AQI |
| MCP | `water-quality-monitoring-mcp` | 水质监测接口，COD/BOD，氨氮，总磷，重金属 |
| MCP | `noise-monitoring-mcp` | 噪声监测接口，声级计数据，频谱分析，达标判断 |
| Skill | `monitoring-report` | 环境监测报告，监测方案，数据整理，统计分析，达标评价，结论建议 |
| Skill | `pollution-treatment-design` | 污染治理方案设计，工艺选择，参数计算，设备选型，投资估算，运行成本，效果预测 |
| Skill | `eia-report-writing` | 环评报告编制，工程分析，现状调查，影响预测，环保措施，公众参与，结论建议 |

---

### 组合包 ENV-02：碳管理与碳中和包

**适用场景：** 企业碳管理部门、碳咨询公司、碳中和服务机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-carbon-management` | 碳管理体系，碳盘查，碳核算，减排目标 |
| 插件 | `dsh-plugin-carbon-consulting` | 碳咨询服务，碳战略，碳路径，碳资产开发 |
| 插件 | `dsh-plugin-esg-reporting` | ESG 报告，ESG 管理，指标体系，信息披露 |
| MCP | `carbon-factor-mcp` | 碳排放因子数据库接口，国家因子，区域因子，行业因子 |
| MCP | `carbon-market-data-mcp` | 碳市场数据接口，碳价行情，成交量，CCER 价格 |
| MCP | `esg-database-mcp` | ESG 数据库接口，评级数据，指标体系，行业基准 |
| Skill | `carbon-inventory` | 碳盘查执行，组织边界，运营边界，数据收集，排放计算，不确定性分析，盘查报告 |
| Skill | `carbon-neutrality-roadmap` | 碳中和路径规划，基线设定，情景分析，减排措施，成本效益，时间表，里程碑 |
| Skill | `esg-report-writing` | ESG 报告撰写，治理结构，环境绩效，社会影响，指标体系，案例故事，数据图表 |

---

### 组合包 ENV-03：循环经济与废弃物管理包

**适用场景：** 废弃物处理企业、回收公司、环卫企业、循环经济园区

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-waste-management` | 废弃物管理，收运计划，处理处置，台账记录 |
| 插件 | `dsh-plugin-recycling` | 资源回收，分类回收，再生利用，回收网络 |
| 插件 | `dsh-plugin-circular-economy` | 循环经济，产业链设计，资源效率，模式创新 |
| MCP | `waste-facility-mcp` | 废弃物处理设施接口，焚烧厂，填埋场，生化处理 |
| MCP | `recycling-market-mcp` | 再生资源市场接口，回收价格，供需信息，交易数据 |
| MCP | `hazardous-waste-mcp` | 危废管理接口，危废申报，转移联单，处置跟踪 |
| Skill | `waste-collection-route` | 废弃物收运路线优化，收集点规划，车辆调度，路径优化，成本分析，效率提升 |
| Skill | `recycling-business-model` | 回收商业模式设计，回收品类，定价策略，渠道建设，增值服务，盈利模式，扩张计划 |
| Skill | `circular-economy-design` | 循环经济方案设计，产业链分析，资源循环，产品生态设计，共享经济，产业共生，政策建议 |

---

## 22. 宠物行业

### 组合包 PET-01：宠物医院与诊疗包

**适用场景：** 宠物医院、动物诊所、兽医诊疗机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-veterinary-clinic` | 宠物诊疗管理，病历管理，处方开具，诊疗记录 |
| 插件 | `dsh-plugin-pet-health-record` | 宠物健康档案，疫苗记录，驱虫记录，体检报告 |
| 插件 | `dsh-plugin-veterinary-pharmacy` | 宠物药房管理，药品库存，处方审核，用药指导 |
| MCP | `veterinary-database-mcp` | 兽医数据库接口，疾病库，药品库，诊疗指南 |
| MCP | `lab-result-mcp` | 检验结果接口，血常规，生化，影像，病理 |
| MCP | `pet-microchip-mcp` | 宠物芯片接口，芯片登记，信息查询，失宠寻找 |
| Skill | `veterinary-diagnosis` | 宠物疾病诊断，病史采集，体格检查，实验室检查，鉴别诊断，治疗方案，预后判断 |
| Skill | `treatment-plan-pet` | 宠物治疗方案，用药计划，护理要求，复诊安排，费用预估，主人沟通，注意事项 |
| Skill | `pet-surgery` | 宠物手术管理，术前评估，麻醉方案，手术记录，术后护理，疼痛管理，并发症预防 |

---

### 组合包 PET-02：宠物美容与寄养包

**适用场景：** 宠物美容店、宠物寄养酒店、宠物生活馆

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-pet-grooming` | 宠物美容管理，预约管理，美容项目，造型设计 |
| 插件 | `dsh-plugin-pet-boarding` | 宠物寄养管理，房间管理，寄养计划，日常护理 |
| 插件 | `dsh-plugin-pet-retail` | 宠物用品零售，商品管理，库存管理，会员销售 |
| MCP | `pet-grooming-schedule-mcp` | 美容排期接口，预约管理，技师分配，服务记录 |
| MCP | `pet-hotel-mcp` | 宠物酒店接口，房间管理，入住登记，护理记录 |
| MCP | `pos-pet-mcp` | POS 收银接口，商品销售，会员积分，优惠券 |
| Skill | `pet-grooming-design` | 宠物美容造型设计，品种标准，客户需求，毛发状况，造型方案，护理建议，产品推荐 |
| Skill | `pet-boarding-care` | 宠物寄养护理，入住评估，喂养计划，运动安排，健康监测，行为观察，主人汇报 |
| Skill | `pet-retail-merchandising` | 宠物用品零售运营，选品策略，陈列设计，促销活动，会员运营，库存管理，销售分析 |

---

### 组合包 PET-03：宠物训练与行为包

**适用场景：** 宠物训练师、行为咨询师、训犬学校、宠物学校

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-pet-training` | 宠物训练管理，训练计划，课程安排，训练记录 |
| 插件 | `dsh-plugin-pet-behavior` | 宠物行为咨询，行为评估，行为矫正，主人指导 |
| 插件 | `dsh-plugin-pet-competition` | 宠物赛事管理，比赛报名，训练备战，成绩记录 |
| MCP | `dog-training-mcp` | 训犬接口，训练方法，指令库，行为记录 |
| MCP | `pet-behavior-mcp` | 宠物行为数据库，行为学资料，案例库，评估量表 |
| MCP | `pet-competition-mcp` | 宠物赛事接口，比赛信息，报名管理，成绩查询 |
| Skill | `dog-training-plan` | 训犬计划制定，基础服从，技能训练，行为塑造，训练方法，进度跟踪，主人配合 |
| Skill | `pet-behavior-correction` | 宠物行为矫正，行为评估，问题分析，矫正方案，环境管理，主人培训，效果评估 |
| Skill | `pet-competition-prep` | 宠物赛事备战，比赛项目，训练重点，体能管理，心理调节，赛场适应，比赛策略 |

---

## 23. 母婴与育儿

### 组合包 BABY-01：母婴护理与月子包

**适用场景：** 月子中心、母婴护理机构、产后康复中心、月嫂公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-postpartum-care` | 产后护理管理，产妇护理，月子餐，康复计划 |
| 插件 | `dsh-plugin-newborn-care` | 新生儿护理，喂养记录，睡眠记录，发育监测 |
| 插件 | `dsh-plugin-maternal-health` | 产妇健康管理，产后复查，心理评估，康复训练 |
| MCP | `maternal-health-mcp` | 产妇健康数据接口，体检数据，康复记录，心理评估 |
| MCP | `baby-growth-mcp` | 婴儿生长发育接口，身高体重，头围，发育里程碑 |
| MCP | `nutrition-maternal-mcp` | 母婴营养数据库，月子餐谱，哺乳期营养，辅食添加 |
| Skill | `postpartum-care-plan` | 产后护理计划，伤口护理，子宫恢复，恶露观察，乳房护理，康复训练，心理关怀 |
| Skill | `newborn-care-guide` | 新生儿护理指导，喂养指导，睡眠管理，脐带护理，黄疸观察，抚触按摩，早教启蒙 |
| Skill | `confinement-meal-planning` | 月子餐计划，排期调理，营养搭配，食材选择，烹饪方法，饮食禁忌，个性化调整 |

---

### 组合包 BABY-02：早教与亲子包

**适用场景：** 早教中心、亲子园、托育机构、儿童成长中心

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-early-education` | 早教管理，课程管理，教案设计，教学记录 |
| 插件 | `dsh-plugin-parent-child` | 亲子活动管理，活动策划，亲子课程，家庭指导 |
| 插件 | `dsh-plugin-child-development` | 儿童发展评估，发育测评，能力分析，成长档案 |
| MCP | `early-childhood-mcp` | 早教课程数据库，教案库，活动方案，教具清单 |
| MCP | `child-assessment-mcp` | 儿童评估量表接口，发育筛查，能力评估，气质类型 |
| MCP | `parenting-resource-mcp` | 育儿资源接口，育儿知识，家长课程，专家问答 |
| Skill | `early-education-curriculum` | 早教课程设计，年龄段划分，发展目标，活动方案，教具准备，观察记录，家园共育 |
| Skill | `child-development-assessment` | 儿童发展评估，大运动精细动作，语言认知，社会情绪，适应性，发展商数，干预建议 |
| Skill | `parenting-guidance` | 育儿指导，喂养睡眠，行为习惯，情绪管理，亲子沟通，家庭环境，常见问题 |

---

### 组合包 BABY-03：母婴零售与电商包

**适用场景：** 母婴店、母婴电商、母婴品牌、孕婴童连锁

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-maternity-retail` | 母婴零售管理，商品管理，库存管理，会员管理 |
| 插件 | `dsh-plugin-baby-products` | 母婴产品管理，产品库，产品评测，推荐方案 |
| 插件 | `dsh-plugin-maternity-marketing` | 母婴营销，内容营销，社群运营，活动策划 |
| MCP | `maternity-ecommerce-mcp` | 母婴电商接口，商品上架，订单管理，客户服务 |
| MCP | `baby-product-database-mcp` | 母婴产品数据库，产品信息，成分分析，安全评级 |
| MCP | `maternity-community-mcp` | 母婴社区接口，用户内容，问答互动，专家入驻 |
| Skill | `maternity-product-selection` | 母婴选品策略，品类规划，品牌选择，质量把控，价格带，利润分析，差异化竞争 |
| Skill | `maternity-content-marketing` | 母婴内容营销，育儿知识，产品测评，种草文案，短视频脚本，直播话术，KOL 合作 |
| Skill | `maternity-member-operations` | 母婴会员运营，会员分层，权益设计，成长体系，社群运营，复购提升，转介绍激励 |

---

## 24. 养老与康养

### 组合包 ELDER-01：养老机构与照护包

**适用场景：** 养老院、护理院、养老社区、长者照护中心

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-elderly-care` | 养老照护管理，老人档案，照护计划，护理记录 |
| 插件 | `dsh-plugin-nursing-management` | 护理管理，护理等级，护理排班，护理质量 |
| 插件 | `dsh-plugin-elderly-health` | 老人健康管理，健康档案，慢病管理，用药管理 |
| MCP | `ehr-elderly-mcp` | 老人电子健康档案接口，病历数据，体检报告，用药记录 |
| MCP | `care-plan-mcp` | 照护计划接口，评估量表，照护等级，服务清单 |
| MCP | `smart-elderly-mcp` | 智慧养老接口，智能床垫，跌倒检测，定位手环，紧急呼叫 |
| Skill | `elderly-care-assessment` | 老人照护评估，自理能力，认知功能，营养状况，心理状态，照护等级，服务计划 |
| Skill | `nursing-care-plan` | 护理计划制定，生活照料，医疗护理，康复训练，心理慰藉，临终关怀，质量标准 |
| Skill | `elderly-chronic-disease` | 老人慢病管理，高血压糖尿病，用药管理，饮食指导，运动建议，监测指标，急性事件预防 |

---

### 组合包 ELDER-02：居家养老与社区养老包

**适用场景：** 居家养老服务公司、社区养老服务中心、养老驿站

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-home-care` | 居家养老管理，上门服务，服务计划，服务记录 |
| 插件 | `dsh-plugin-community-elderly` | 社区养老管理，日间照料，助餐助浴，文娱活动 |
| 插件 | `dsh-plugin-elderly-family` | 家属沟通，远程探视，健康汇报，紧急通知 |
| MCP | `home-care-schedule-mcp` | 居家养老排期接口，服务人员，服务项目，时间安排 |
| MCP | `community-center-mcp` | 社区养老中心接口，场地管理，活动管理，助餐管理 |
| MCP | `family-communication-mcp` | 家属沟通接口，视频探视，消息推送，健康报告 |
| Skill | `home-care-service` | 居家养老服务，需求评估，服务设计，人员匹配，服务标准，质量监控，满意度调查 |
| Skill | `community-elderly-activities` | 社区养老活动，文娱活动，健康讲座，兴趣小组，节日庆祝，志愿者服务，社会参与 |
| Skill | `elderly-family-communication` | 老人家属沟通，健康汇报，生活照料，心理状态，紧急事件，沟通技巧，矛盾调解 |

---

### 组合包 ELDER-03：康复理疗与适老化包

**适用场景：** 康复中心、老年康复机构、适老化改造公司、辅具公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-elderly-rehab` | 老年康复管理，康复评估，康复计划，康复训练 |
| 插件 | `dsh-plugin-aging-in-place` | 适老化改造，居家评估，改造方案，产品推荐 |
| 插件 | `dsh-plugin-assistive-device` | 辅具管理，辅具评估，辅具适配，使用指导 |
| MCP | `rehab-equipment-elderly-mcp` | 老年康复设备接口，训练设备，评估设备，数据记录 |
| MCP | `home-assessment-mcp` | 居家环境评估接口，评估量表，风险识别，改造建议 |
| MCP | `assistive-technology-mcp` | 辅助技术数据库，辅具产品，适配指南，使用说明 |
| Skill | `elderly-rehabilitation` | 老年康复方案，功能评估，康复目标，训练计划，物理治疗，作业治疗，言语治疗 |
| Skill | `aging-in-place-design` | 适老化改造设计，居家评估，风险识别，空间改造，卫浴改造，厨房改造，智能设备，成本预算 |
| Skill | `assistive-device-prescription` | 辅具适配处方，功能评估，需求分析，辅具选择，适配调整，使用训练，维护保养，费用报销 |

---

## 25. 咨询与专业服务

### 组合包 CONSULT-01：管理咨询与战略包

**适用场景：** 管理咨询公司、战略咨询顾问、企业顾问

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-strategy-consulting` | 战略咨询管理，项目管理，分析框架，报告撰写 |
| 插件 | `dsh-plugin-management-consulting` | 管理咨询，组织设计，流程优化，绩效提升 |
| 插件 | `dsh-plugin-consulting-research` | 咨询研究，行业研究，市场调研，数据分析 |
| MCP | `consulting-framework-mcp` | 咨询框架数据库，分析模型，方法论，工具模板 |
| MCP | `industry-research-mcp` | 行业研究数据库，行业报告，市场数据，竞争情报 |
| MCP | `consulting-document-mcp` | 咨询文档接口，报告模板，PPT 模板，图表工具 |
| Skill | `strategy-analysis` | 战略分析，外部环境，内部能力，SWOT 分析，竞争格局，战略选择，实施路径 |
| Skill | `organizational-design` | 组织设计，组织结构，部门职责，岗位编制，管理幅度，汇报关系，变革管理 |
| Skill | `business-process-optimization` | 业务流程优化，流程梳理，瓶颈识别，价值流分析，流程再造，系统支撑，效果评估 |

---

### 组合包 CONSULT-02：IT 咨询与数字化包

**适用场景：** IT 咨询公司、数字化转型顾问、信息化规划专家

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-digital-transformation` | 数字化转型咨询，转型规划，能力评估，路线图 |
| 插件 | `dsh-plugin-it-consulting` | IT 咨询，IT 规划，系统选型，架构设计 |
| 插件 | `dsh-plugin-it-governance` | IT 治理，IT 管控，IT 审计，信息安全 |
| MCP | `enterprise-architecture-mcp` | 企业架构框架接口，TOGAF，业务架构，应用架构，数据架构，技术架构 |
| MCP | `it-system-database-mcp` | IT 系统数据库，系统清单，功能对比，厂商信息，价格参考 |
| MCP | `cybersecurity-framework-mcp` | 网络安全框架接口，等保 2.0，ISO 27001，NIST，控制措施 |
| Skill | `digital-transformation-roadmap` | 数字化转型路线图，现状评估，愿景目标，能力差距，优先级，项目组合，实施节奏，价值衡量 |
| Skill | `it-system-selection` | IT 系统选型，需求分析，功能清单，厂商调研，方案对比，POC 测试，商务谈判，决策建议 |
| Skill | `it-governance-design` | IT 治理设计，治理组织，决策机制，流程制度，绩效管理，风险管理，合规审计，持续改进 |

---

### 组合包 CONSULT-03：人力资源咨询包

**适用场景：** HR 咨询公司、组织发展顾问、人才管理专家

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-hr-consulting` | HR 咨询管理，HR 诊断，体系设计，项目实施 |
| 插件 | `dsh-plugin-talent-management` | 人才管理咨询，人才盘点，胜任力模型，发展通道 |
| 插件 | `dsh-plugin-od-consulting` | 组织发展咨询，组织诊断，文化建设，变革管理 |
| MCP | `hr-benchmark-mcp` | HR 基准数据接口，薪酬调研，组织效能，人才指标 |
| MCP | `competency-database-mcp` | 胜任力数据库，胜任力模型，行为指标，评估工具 |
| MCP | `org-diagnosis-mcp` | 组织诊断工具接口，诊断问卷，分析模型，报告模板 |
| Skill | `hr-system-design` | HR 体系设计，招聘体系，培训体系，薪酬体系，绩效体系，员工关系，制度流程 |
| Skill | `talent-review` | 人才盘点，九宫格，潜力评估，绩效评估，关键人才，继任计划，发展建议，人才地图 |
| Skill | `organizational-culture` | 组织文化建设，文化诊断，文化定位，价值观提炼，行为准则，文化落地，效果评估 |

---

## 26. 会计审计与税务

### 组合包 ACCT-01：会计核算与财务包

**适用场景：** 会计师事务所、代理记账公司、企业财务部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-accounting` | 会计核算管理，凭证管理，账簿管理，报表生成 |
| 插件 | `dsh-plugin-financial-reporting` | 财务报告管理，财报编制，附注披露，财报分析 |
| 插件 | `dsh-plugin-bookkeeping` | 代理记账管理，客户管理，票据处理，纳税申报 |
| MCP | `accounting-software-mcp` | 财务软件接口（用友/金蝶/SAP/Oracle），凭证，账簿，报表 |
| MCP | `invoice-mcp` | 发票接口，发票识别，验真查重，进项销项，电子发票 |
| MCP | `bank-statement-mcp` | 银行流水接口，对账单，交易明细，对账核销 |
| Skill | `financial-statement-preparation` | 财务报表编制，资产负债表，利润表，现金流量表，所有者权益变动表，附注，合并报表 |
| Skill | `month-end-close` | 月末结账流程，凭证审核，计提摊销，结转损益，对账调账，结账检查，报表出具 |
| Skill | `financial-analysis` | 财务分析，比率分析，趋势分析，结构分析，杜邦分析，预算差异，经营建议 |

---

### 组合包 ACCT-02：审计与内控包

**适用场景：** 审计师事务所、内部审计部门、内控咨询公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-audit-management` | 审计项目管理，审计计划，审计程序，工作底稿 |
| 插件 | `dsh-plugin-internal-control` | 内部控制管理，内控评价，风险评估，控制测试 |
| 插件 | `dsh-plugin-audit-reporting` | 审计报告管理，报告撰写，意见类型，管理建议 |
| MCP | `audit-working-paper-mcp` | 审计工作底稿接口，底稿模板，索引编号，复核记录 |
| MCP | `internal-control-framework-mcp` | 内控框架接口，COSO，企业内部控制基本规范，控制矩阵 |
| MCP | `audit-evidence-mcp` | 审计证据接口，函证，盘点，检查，观察，询问，分析程序 |
| Skill | `audit-planning` | 审计计划编制，业务了解，风险评估，重要性水平，审计策略，审计程序，时间预算，人员安排 |
| Skill | `substantive-testing` | 实质性测试，细节测试，分析程序，抽样方法，证据评价，差异调查，审计调整，结论形成 |
| Skill | `internal-control-evaluation` | 内部控制评价，控制设计，控制运行，缺陷识别，缺陷评级，整改建议，内控报告，管理建议书 |

---

### 组合包 ACCT-03：税务筹划与合规包

**适用场景：** 税务师事务所、税务咨询公司、企业税务部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-tax-planning` | 税务筹划管理，筹划方案，税负测算，风险评估 |
| 插件 | `dsh-plugin-tax-compliance` | 税务合规管理，纳税申报，发票管理，税务档案 |
| 插件 | `dsh-plugin-tax-consulting` | 税务咨询，政策解读，实务操作，争议解决 |
| MCP | `tax-policy-mcp` | 税收政策数据库接口，法律法规，政策文件，解读分析 |
| MCP | `tax-filing-mcp` | 纳税申报接口，电子税务局，申报表单，申报数据 |
| MCP | `transfer-pricing-mcp` | 转让定价接口，可比性分析，基准研究，文档要求 |
| Skill | `tax-planning-scheme` | 税务筹划方案，业务分析，政策适用，方案设计，税负测算，风险分析，实施步骤，效果预估 |
| Skill | `tax-risk-assessment` | 税务风险评估，风险识别，风险评级，风险矩阵，应对措施，内控建议，自查报告，整改方案 |
| Skill | `tax-dispute-resolution` | 税务争议解决，案情分析，法律依据，证据组织，沟通策略，听证申辩，行政复议，行政诉讼 |

---

## 27. 翻译与本地化

### 组合包 TRANS-01：翻译服务与项目包

**适用场景：** 翻译公司、翻译团队、自由译者、本地化公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-translation-project` | 翻译项目管理，项目报价，任务分配，进度跟踪 |
| 插件 | `dsh-plugin-translation-memory` | 翻译记忆管理，TM 库，术语库，重复利用 |
| 插件 | `dsh-plugin-translation-quality` | 翻译质量管理，QA 检查，错误统计，质量评分 |
| MCP | `cat-tool-mcp` | CAT 工具接口（Trados/memoQ/Wordfast），项目文件，翻译记忆，术语库 |
| MCP | `machine-translation-mcp` | 机器翻译接口（DeepL/Google/百度/有道），MT 翻译，译后编辑 |
| MCP | `terminology-mcp` | 术语库接口，术语管理，术语提取，术语一致性 |
| Skill | `translation-project-management` | 翻译项目管理，需求分析，字数统计，报价方案，团队组建，排期计划，质量控制，交付验收 |
| Skill | `translation-quality-assurance` | 翻译质量保证，QA 检查清单，错误分类，抽样检查，评分标准，反馈机制，改进措施，译者评估 |
| Skill | `localization-engineering` | 本地化工程，文件解析，格式处理，资源提取，翻译集成，编译测试，缺陷修复，交付打包 |

---

### 组合包 TRANS-02：本地化与国际化包

**适用场景：** 软件本地化公司、游戏本地化、网站本地化、产品国际化团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-software-localization` | 软件本地化，资源文件，界面翻译，测试验证 |
| 插件 | `dsh-plugin-website-localization` | 网站本地化，多语言网站，SEO 本地化，内容翻译 |
| 插件 | `dsh-plugin-game-localization` | 游戏本地化，文本翻译，文化适配，配音字幕，LQA |
| MCP | `i18n-framework-mcp` | 国际化框架接口（i18next/react-intl/vue-i18n），资源管理，语言切换 |
| MCP | `cms-multilingual-mcp` | 多语言 CMS 接口，内容管理，多语言发布，翻译工作流 |
| MCP | `game-localization-mcp` | 游戏本地化接口，文本提取，字符串管理，文化适配检查 |
| Skill | `software-localization-process` | 软件本地化流程，资源提取，翻译管理，UI 适配，功能测试，Bug 修复，多语言发布，持续维护 |
| Skill | `cultural-adaptation` | 文化适配，文化差异分析，内容调整，禁忌规避，本地化创意，用户体验，市场接受度，品牌一致性 |
| Skill | `multilingual-seo` | 多语言 SEO，关键词研究，元数据优化，URL 结构，hreflang 标签，内容本地化，链接建设，排名跟踪 |

---

### 组合包 TRANS-03：口译与会议服务包

**适用场景：** 口译公司、会议服务公司、同传译员、交传译员

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-consecutive-interpreting` | 交替传译管理，会议准备，术语准备，口译记录 |
| 插件 | `dsh-plugin-simultaneous-interpreting` | 同声传译管理，同传设备，同传箱，同传团队 |
| 插件 | `dsh-plugin-conference-services` | 会议服务管理，会议策划，设备租赁，人员安排 |
| MCP | `conference-management-mcp` | 会议管理接口，会议日程，参会人员，会议资料 |
| MCP | `interpreting-equipment-mcp` | 口译设备接口，同传设备，导览设备，投票设备 |
| MCP | `speech-to-text-mcp` | 语音转文字接口，实时转写，会议记录，字幕生成 |
| Skill | `conference-preparation` | 会议口译准备，背景调研，术语准备，资料预读，演讲者分析，场景模拟，团队协调，应急预案 |
| Skill | `simultaneous-interpreting` | 同声传译执行，听辨理解，记忆转换，表达输出，团队配合，设备操作，质量监控，应急处理 |
| Skill | `conference-service-planning` | 会议服务策划，会议方案，场地选择，设备配置，人员安排，流程设计，预算编制，现场执行 |

---

## 28. 婚庆与礼仪

### 组合包 WED-01：婚礼策划与执行包

**适用场景：** 婚礼策划公司、婚礼策划师、婚庆工作室

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-wedding-planning` | 婚礼策划管理，方案设计，预算编制，流程安排 |
| 插件 | `dsh-plugin-wedding-execution` | 婚礼执行管理，当天流程，人员调度，现场协调 |
| 插件 | `dsh-plugin-wedding-vendor` | 婚礼供应商管理，场地，摄影，化妆，花艺，司仪 |
| MCP | `wedding-venue-mcp` | 婚礼场地接口，场地信息，档期查询，价格查询，预订管理 |
| MCP | `wedding-vendor-mcp` | 婚礼供应商数据库，供应商信息，服务项目，评价评分，案例展示 |
| MCP | `wedding-inspiration-mcp` | 婚礼灵感库，风格参考，配色方案，花艺设计，布置案例 |
| Skill | `wedding-concept-design` | 婚礼概念设计，新人故事，主题定位，风格选择，色彩搭配，空间设计，仪式流程，情感表达 |
| Skill | `wedding-budget-planning` | 婚礼预算编制，预算分配，项目明细，供应商报价，成本控制，超支预警，结算管理，性价比分析 |
| Skill | `wedding-day-coordination` | 婚礼当天执行，流程把控，人员调度，供应商协调，突发处理，时间管理，新人关怀，完美交付 |

---

### 组合包 WED-02：婚纱摄影与影像包

**适用场景：** 婚纱摄影工作室、摄影团队、影像制作公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-wedding-photography` | 婚纱摄影管理，拍摄计划，拍摄流程，选片管理 |
| 插件 | `dsh-plugin-photo-retouching` | 照片修图管理，修图标准，修图流程，质量检查 |
| 插件 | `dsh-plugin-wedding-video` | 婚礼视频管理，视频拍摄，剪辑制作，成品交付 |
| MCP | `photo-management-mcp` | 照片管理接口，图库管理，相册制作，在线选片 |
| MCP | `photo-editing-mcp` | 照片编辑接口，批量处理，滤镜预设，AI 修图 |
| MCP | `video-editing-mcp` | 视频编辑接口，剪辑软件，素材管理，特效模板 |
| Skill | `wedding-photography-plan` | 婚纱摄影方案，风格定位，场景选择，服装搭配，拍摄流程，光线设计，姿势引导，情感捕捉 |
| Skill | `photo-retouching-workflow` | 照片修图流程，初修精修，调色风格，皮肤处理，身材修饰，背景优化，批量处理，质量标准 |
| Skill | `wedding-film-production` | 婚礼影片制作，拍摄脚本，镜头设计，素材整理，剪辑节奏，音乐选择，调色包装，情感叙事 |

---

### 组合包 WED-03：庆典礼仪与活动包

**适用场景：** 庆典公司、礼仪公司、活动策划公司、商业活动团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-ceremony-planning` | 庆典策划管理，方案设计，流程编排，仪式设计 |
| 插件 | `dsh-plugin-protocol-etiquette` | 礼仪管理，礼仪规范，人员培训，现场执行 |
| 插件 | `dsh-plugin-corporate-event` | 企业活动管理，年会，发布会，颁奖典礼，团建 |
| MCP | `event-venue-mcp` | 活动场地接口，酒店，会议中心，展览场馆，户外场地 |
| MCP | `event-supplier-mcp` | 活动供应商数据库，搭建，灯光音响，演艺，主持 |
| MCP | `event-registration-mcp` | 活动报名接口，签到管理，票务销售，嘉宾管理 |
| Skill | `ceremony-ritual-design` | 庆典仪式设计，仪式流程，环节设计，象征意义，文化内涵，情感升华，视觉呈现，音乐配合 |
| Skill | `corporate-annual-meeting` | 企业年会策划，主题设计，节目编排，流程安排，互动环节，颁奖设计，预算管理，执行保障 |
| Skill | `product-launch-event` | 产品发布会策划，发布会主题，场地选择，舞台设计，流程安排，媒体邀请，传播策略，效果评估 |

---

## 29. 美容美业

### 组合包 BEAUTY-01：美容护肤与皮肤管理包

**适用场景：** 美容院、皮肤管理中心、医美机构、护肤工作室

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-skin-management` | 皮肤管理，皮肤检测，护理方案，疗程管理 |
| 插件 | `dsh-plugin-beauty-treatment` | 美容项目管理，项目库，操作流程，效果跟踪 |
| 插件 | `dsh-plugin-skincare-product` | 护肤产品管理，产品库，成分分析，产品推荐 |
| MCP | `skin-analysis-mcp` | 皮肤检测接口，皮肤检测仪，AI 肤诊，检测报告 |
| MCP | `skincare-ingredient-mcp` | 护肤成分数据库，成分功效，安全性，搭配禁忌，敏感风险 |
| MCP | `beauty-device-mcp` | 美容设备接口，光电设备，射频仪，水光仪，操作记录 |
| Skill | `skin-analysis-report` | 皮肤检测报告，肤质类型，问题分析，成因解读，严重程度，护理重点，产品建议，生活方式 |
| Skill | `facial-treatment-plan` | 面部护理方案，项目选择，疗程设计，操作步骤，产品搭配，居家护理，效果预期，注意事项 |
| Skill | `skincare-product-recommendation` | 护肤产品推荐，肤质匹配，成分分析，功效需求，预算范围，产品组合，使用顺序，替代方案 |

---

### 组合包 BEAUTY-02：美发造型与美甲包

**适用场景：** 美发沙龙、美甲店、美睫店、造型工作室

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-hair-salon` | 美发管理，发型设计，烫染管理，造型记录 |
| 插件 | `dsh-plugin-nail-art` | 美甲管理，款式设计，美甲记录，作品管理 |
| 插件 | `dsh-plugin-beauty-booking` | 美业预约管理，预约排期，技师分配，到店管理 |
| MCP | `hair-color-mcp` | 染发配方接口，色板，配方库，调色计算，效果预览 |
| MCP | `nail-design-mcp` | 美甲设计库，款式图库，教程，素材，灵感 |
| MCP | `salon-booking-mcp` | 美业预约系统接口，在线预约，排班管理，会员管理 |
| Skill | `hair-style-design` | 发型设计，脸型分析，发质评估，风格匹配，发型建议，发色推荐，烫染方案，日常打理 |
| Skill | `nail-art-design` | 美甲款式设计，甲型选择，配色方案，图案设计，风格定位，季节主题，场合适配，操作步骤 |
| Skill | `salon-operations` | 美发沙龙运营，排班管理，技师绩效，客户管理，库存管理，促销活动，服务流程，客户体验 |

---

### 组合包 BEAUTY-03：医美与轻医美包

**适用场景：** 医美机构、轻医美诊所、皮肤医美中心、医疗美容医院

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-medical-aesthetics` | 医美管理，咨询设计，项目管理，诊疗记录 |
| 插件 | `dsh-plugin-aesthetics-consultation` | 医美咨询，面诊设计，方案制定，客户沟通 |
| 插件 | `dsh-plugin-aesthetics-aftercare` | 医美术后管理，恢复跟踪，护理指导，效果评估 |
| MCP | `aesthetics-project-mcp` | 医美项目数据库，项目信息，适应症，禁忌症，价格，恢复期 |
| MCP | `before-after-photo-mcp` | 术前术后照片接口，照片管理，对比分析，效果评估 |
| MCP | `medical-record-mcp` | 医美病历接口，电子病历，诊疗记录，知情同意，处方管理 |
| Skill | `aesthetics-consultation-design` | 医美咨询设计，需求分析，美学评估，方案设计，项目组合，价格方案，风险告知，预期管理 |
| Skill | `aesthetics-treatment-plan` | 医美治疗方案，项目选择，治疗顺序，参数设置，操作规范，麻醉管理，应急处理，质量控制 |
| Skill | `aesthetics-post-care` | 医美术后护理，恢复指导，护理产品，注意事项，复查安排，并发症预防，效果评估，满意度管理 |

---

## 30. 殡葬与生命服务

### 组合包 FUNERAL-01：殡葬服务与礼仪包

**适用场景：** 殡仪馆、殡葬服务公司、礼仪服务公司、陵园

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-funeral-service` | 殡葬服务管理，服务套餐，流程安排，人员调度 |
| 插件 | `dsh-plugin-funeral-ritual` | 殡葬礼仪管理，仪式设计，流程编排，礼仪执行 |
| 插件 | `dsh-plugin-funeral-products` | 殡葬用品管理，寿衣，骨灰盒，花圈，随葬品 |
| MCP | `funeral-home-mcp` | 殡仪馆接口，厅房管理，火化预约，服务项目，价格查询 |
| MCP | `cemetery-mcp` | 陵园接口，墓位管理，选墓系统，价格查询，预约祭扫 |
| MCP | `funeral-product-mcp` | 殡葬用品数据库，产品信息，材质工艺，价格，定制服务 |
| Skill | `funeral-service-planning` | 殡葬服务方案，需求了解，服务选择，流程安排，人员配置，物品准备，预算编制，家属沟通 |
| Skill | `memorial-ceremony-design` | 告别仪式设计，仪式主题，流程编排，音乐选择，视频制作，花艺布置，悼词撰写，情感表达 |
| Skill | `funeral-ritual-etiquette` | 殡葬礼仪规范，传统礼仪，现代礼仪，宗教礼仪，地域习俗，亲属礼节，吊唁礼节，谢礼流程 |

---

### 组合包 FUNERAL-02：陵园与墓地管理包

**适用场景：** 陵园公司、公墓管理处、墓地销售、殡葬地产

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-cemetery-sales` | 陵园销售管理，墓位销售，客户跟进，合同签订 |
| 插件 | `dsh-plugin-cemetery-management` | 陵园运营管理，墓位管理，维护管理，祭扫管理 |
| 插件 | `dsh-plugin-memorial-architecture` | 纪念建筑设计，墓碑设计，家族墓，艺术墓 |
| MCP | `cemetery-gis-mcp` | 陵园 GIS 接口，墓区地图，墓位定位，选墓可视化 |
| MCP | `tombstone-design-mcp` | 墓碑设计库，款式图库，材质工艺，雕刻图案，定制设计 |
| MCP | `cemetery-maintenance-mcp` | 陵园维护接口，绿化养护，清洁管理，维修记录，巡检管理 |
| Skill | `cemetery-sales-consultation` | 陵园销售咨询，需求分析，风水介绍，墓位推荐，价格方案，合同条款，售后服务，客户维护 |
| Skill | `tombstone-custom-design` | 墓碑定制设计，风格定位，材质选择，尺寸规格，雕刻内容，碑文撰写，工艺说明，效果预览 |
| Skill | `cemetery-operations-management` | 陵园运营管理，墓区规划，销售管理，维护管理，祭扫管理，安全管理，环境管理，品牌建设 |

---

### 组合包 FUNERAL-03：生命教育与哀伤辅导包

**适用场景：** 生命教育机构、哀伤辅导中心、心理咨询机构、临终关怀机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-life-education` | 生命教育管理，课程设计，活动策划，教材管理 |
| 插件 | `dsh-plugin-grief-counseling` | 哀伤辅导管理，个案管理，辅导计划，团体辅导 |
| 插件 | `dsh-plugin-hospice-care` | 临终关怀管理，安宁疗护，心理支持，家属陪伴 |
| MCP | `grief-assessment-mcp` | 哀伤评估接口，评估量表，阶段判断，风险筛查 |
| MCP | `life-education-resource-mcp` | 生命教育资源库，课程教案，活动方案，绘本影视 |
| MCP | `hospice-care-mcp` | 临终关怀接口，安宁疗护指南，疼痛管理，心理支持 |
| Skill | `life-education-curriculum` | 生命教育课程设计，年龄分段，主题设计，教学目标，活动方案，教学方法，评估方式，家校协同 |
| Skill | `grief-counseling-session` | 哀伤辅导个案，初次访谈，哀伤评估，阶段判断，辅导目标，干预方法，家庭支持，转介机制，效果评估 |
| Skill | `hospice-care-plan` | 临终关怀计划，需求评估，疼痛管理，症状控制，心理支持，灵性关怀，家属陪伴，临终准备，哀伤随访 |

---

## 补充篇总结

### 新增行业与组合包统计

| 行业 | 组合包数 | 核心场景 |
|------|----------|----------|
| 汽车行业 | 3 | 汽车销售、维修保养、制造供应链 |
| 能源与公用事业 | 3 | 电力电网、新能源、油气水务燃气 |
| 体育与健身 | 3 | 健身俱乐部、体育赛事、运动康复 |
| 文化艺术与文博 | 3 | 博物馆、艺术画廊、演出演艺 |
| 游戏与电竞 | 3 | 游戏开发、电竞俱乐部、游戏运营 |
| 环保与碳中和 | 3 | 环境监测、碳管理、循环经济 |
| 宠物行业 | 3 | 宠物医院、美容寄养、训练行为 |
| 母婴与育儿 | 3 | 月子护理、早教亲子、母婴零售 |
| 养老与康养 | 3 | 养老机构、居家社区、康复适老 |
| 咨询与专业服务 | 3 | 管理咨询、IT 咨询、HR 咨询 |
| 会计审计与税务 | 3 | 会计核算、审计内控、税务筹划 |
| 翻译与本地化 | 3 | 翻译服务、软件本地化、口译会议 |
| 婚庆与礼仪 | 3 | 婚礼策划、婚纱摄影、庆典活动 |
| 美容美业 | 3 | 皮肤管理、美发美甲、医美轻医美 |
| 殡葬与生命服务 | 3 | 殡葬礼仪、陵园墓地、生命教育 |

**补充篇合计：15 个行业 × 3 个组合包 = 45 个组合包**

**主文档 + 补充篇合计：30 个行业 × 3 个组合包 + 4 个通用包 = 94 个组合包**

---

**文档结束**

> 本文档为 DSH 各行业插件/MCP/Skill 组合包大全补充篇，与主文档配合使用。组合包列表将持续更新，建议定期查看最新版本。
