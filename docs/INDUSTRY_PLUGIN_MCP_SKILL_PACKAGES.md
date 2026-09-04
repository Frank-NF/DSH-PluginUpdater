# DSH 各行业插件 / MCP / Skill 组合包大全

> 版本：v1.0.0  
> 日期：2026-09-04  
> 说明：本文档按行业分类整理 DSH Agent 生态所需的插件（Plugin）、MCP 服务器（Model Context Protocol）和技能包（Skill），每个组合包可独立安装使用，也可组合叠加。

---

## 目录

- [通用基础组合包（所有行业必备）](#通用基础组合包所有行业必备)
- [1. 医疗健康行业](#1-医疗健康行业)
- [2. 法律合规行业](#2-法律合规行业)
- [3. 金融财经行业](#3-金融财经行业)
- [4. 教育培训行业](#4-教育培训行业)
- [5. 电商零售行业](#5-电商零售行业)
- [6. 制造业 / 工业](#6-制造业--工业)
- [7. 房地产 / 建筑](#7-房地产--建筑)
- [8. 媒体 / 内容创作](#8-媒体--内容创作)
- [9. 政府 / 公共服务](#9-政府--公共服务)
- [10. IT / 软件开发](#10-it--软件开发)
- [11. 农业](#11-农业)
- [12. 物流 / 供应链](#12-物流--供应链)
- [13. 旅游 / 酒店](#13-旅游--酒店)
- [14. 餐饮行业](#14-餐饮行业)
- [15. 人力资源](#15-人力资源)
- [组合包安装配置指南](#组合包安装配置指南)

---

## 通用基础组合包（所有行业必备）

### 组合包 G-01：文件与办公基础包

**适用场景：** 所有行业的日常办公、文档处理、数据表格操作

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-filesystem` | 本地文件系统操作，文件读写、目录管理、搜索 |
| 插件 | `dsh-plugin-office` | Office 文档处理（Word/Excel/PPT/PDF） |
| 插件 | `dsh-plugin-clipboard` | 剪贴板管理，文本/图片/文件复制粘贴 |
| MCP | `filesystem` | 官方文件系统 MCP，安全的文件读写访问 |
| MCP | `excel-mcp` | Excel 工作簿读写、公式计算、图表生成、数据透视 |
| MCP | `pptx-mcp` | PowerPoint 演示文稿创建与编辑 |
| MCP | `pdf-mcp` | PDF 读取、提取、合并、拆分、表单填写 |
| Skill | `document-processing` | 文档解析、格式转换、内容提取、摘要生成 |
| Skill | `data-analysis-basic` | 表格数据清洗、统计分析、图表生成 |
| Skill | `report-generation` | 报告模板填充、自动排版、导出多格式 |

---

### 组合包 G-02：网络与信息检索包

**适用场景：** 所有行业的信息搜集、网页浏览、数据采集

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-browser` | 浏览器自动化，网页操作、表单填写、截图 |
| 插件 | `dsh-plugin-web-search` | 多搜索引擎集成（百度/谷歌/Bing） |
| 插件 | `dsh-plugin-rss` | RSS 订阅管理，资讯聚合 |
| MCP | `firecrawl-mcp` | 网页爬取、站点地图、结构化数据提取 |
| MCP | `fetch-mcp` | 网页内容获取，转 Markdown 格式 |
| MCP | `brave-search-mcp` | Brave 搜索引擎 API 集成 |
| Skill | `web-research` | 主题研究、多源信息整合、来源标注 |
| Skill | `data-scraping` | 网页数据采集、结构化提取、去重清洗 |
| Skill | `competitor-monitor` | 竞品网站监控、变更检测、差异报告 |

---

### 组合包 G-03：通信与协作包

**适用场景：** 所有行业的团队协作、消息通知、会议管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-email` | 邮件收发、模板管理、自动回复、附件处理 |
| 插件 | `dsh-plugin-calendar` | 日历管理、日程安排、会议预约、提醒 |
| 插件 | `dsh-plugin-notification` | 系统通知、桌面弹窗、声音提醒 |
| MCP | `gmail-mcp` | Gmail 邮件操作（读取/发送/搜索/标签） |
| MCP | `slack-mcp` | Slack 消息发送、频道管理、文件分享 |
| MCP | `notion-mcp` | Notion 页面/数据库读写、内容管理 |
| MCP | `google-calendar-mcp` | Google 日历事件管理 |
| Skill | `meeting-notes` | 会议记录整理、待办提取、行动项分配 |
| Skill | `email-drafting` | 邮件撰写、语气调整、多语言翻译 |
| Skill | `task-management` | 任务创建、优先级排序、进度跟踪、到期提醒 |

---

### 组合包 G-04：数据与数据库包

**适用场景：** 所有行业的数据存储、查询、分析、可视化

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-database` | 多数据库连接管理，SQL 执行，数据浏览 |
| 插件 | `dsh-plugin-chart` | 数据可视化，图表生成（柱状/折线/饼图/热力图） |
| MCP | `postgres-mcp` | PostgreSQL 数据库连接、Schema 检查、安全查询 |
| MCP | `sqlite-mcp` | SQLite 数据库操作，内置分析功能 |
| MCP | `duckdb-mcp` | DuckDB 列式数据库，高性能分析查询 |
| MCP | `mysql-mcp` | MySQL 数据库连接与操作 |
| MCP | `bigquery-mcp` | Google BigQuery 数据仓库查询 |
| Skill | `sql-generation` | 自然语言转 SQL，查询优化，结果解释 |
| Skill | `data-modeling` | 表结构设计、ER 图生成、索引优化建议 |
| Skill | `etl-pipeline` | 数据抽取、转换、加载，数据清洗流水线 |

---

## 1. 医疗健康行业

### 组合包 MED-01：临床文档与病历管理包

**适用场景：** 医院、诊所、口腔门诊的病历书写、临床文档管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-medical-record` | 电子病历模板管理，病历书写辅助，结构化录入 |
| 插件 | `dsh-plugin-medical-terminology` | 医学术语库，ICD-10/CPT 编码查询，术语标准化 |
| 插件 | `dsh-plugin-voice-input` | 语音转文字，门诊对话录音，实时转写 |
| MCP | `fhir-mcp` | FHIR 医疗数据标准接口，患者数据查询 |
| MCP | `medical-literature-mcp` | PubMed/医学文献检索，论文摘要获取 |
| MCP | `ehr-mcp` | 电子健康记录系统集成，患者信息读取 |
| Skill | `clinical-note-generation` | 门诊病历自动生成，SOAP 格式整理，主诉/现病史/查体/诊断 |
| Skill | `medical-coding` | ICD-10 诊断编码、CPT 操作编码自动匹配 |
| Skill | `dental-chart` | 口腔牙位图记录，治疗计划生成，收费项目关联 |

---

### 组合包 MED-02：健康管理与慢病跟踪包

**适用场景：** 健康管理公司、体检中心、慢病管理机构

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-health-data` | 健康数据采集，体检报告解析，指标跟踪 |
| 插件 | `dsh-plugin-wearable` | 可穿戴设备数据同步（手环/血压计/血糖仪） |
| 插件 | `dsh-plugin-diet-nutrition` | 饮食记录，营养计算，膳食建议 |
| MCP | `health-kit-mcp` | 健康数据平台接口，体检数据导入 |
| MCP | `drug-database-mcp` | 药品数据库查询，说明书获取，相互作用检查 |
| MCP | `lab-result-mcp` | 检验报告解析，指标异常标记，参考范围对比 |
| Skill | `health-assessment` | 健康风险评估，体检报告解读，异常指标分析 |
| Skill | `chronic-disease-management` | 高血压/糖尿病/高血脂慢病管理计划，用药提醒，复查跟踪 |
| Skill | `diet-plan-generation` | 个性化饮食方案生成，营养配比计算，食谱推荐 |

---

### 组合包 MED-03：医疗营销与患者运营包

**适用场景：** 民营医院、口腔门诊、医美机构的患者获客与运营

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-patient-crm` | 患者客户管理，就诊记录，消费历史，跟进记录 |
| 插件 | `dsh-plugin-medical-marketing` | 医疗营销内容生成，科普文章，短视频脚本 |
| 插件 | `dsh-plugin-appointment` | 预约挂号管理，排班设置，到诊提醒 |
| MCP | `wechat-mcp` | 微信公众号/企业微信消息推送，客户管理 |
| MCP | `douyin-mcp` | 抖音短视频发布，评论管理，数据统计 |
| MCP | `xiaohongshu-mcp` | 小红书笔记发布，互动管理 |
| Skill | `patient-follow-up` | 术后/治疗后随访，恢复情况跟踪，满意度调查 |
| Skill | `medical-content-marketing` | 医疗科普内容创作，选题策划，多平台分发 |
| Skill | `conversion-optimization` | 咨询到诊转化分析，话术优化，复购提升方案 |

---

## 2. 法律合规行业

### 组合包 LAW-01：合同审查与起草包

**适用场景：** 律师事务所、企业法务部门的合同管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-contract-review` | 合同审查，风险条款识别，修改建议，批注生成 |
| 插件 | `dsh-plugin-contract-template` | 合同模板库管理，模板填充，版本对比 |
| 插件 | `dsh-plugin-legal-research` | 法律检索，法条查询，案例检索 |
| MCP | `westlaw-mcp` | Westlaw 法律数据库检索（英美法） |
| MCP | `pku-law-mcp` | 北大法宝法律数据库（中国法） |
| MCP | `sec-edgar-mcp` | SEC EDGAR 上市公司文件检索 |
| Skill | `contract-risk-assessment` | 合同风险评级，风险点清单，修改建议优先级 |
| Skill | `contract-drafting` | 根据交易要点起草合同，条款生成，风险规避 |
| Skill | `contract-comparison` | 合同版本对比，变更点识别，差异分析报告 |

---

### 组合包 LAW-02：诉讼与案件管理包

**适用场景：** 诉讼律师、律所案件管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-case-management` | 案件管理，当事人信息，证据目录，时间线 |
| 插件 | `dsh-plugin-legal-document` | 法律文书生成，起诉状/答辩状/代理词模板 |
| 插件 | `dsh-plugin-evidence` | 证据管理，证据编号，证明目的，质证意见 |
| MCP | `court-document-mcp` | 裁判文书网检索，判例查询 |
| MCP | `docket-mcp` | 案件 docket 查询，开庭信息，法院公告 |
| Skill | `legal-research-deep` | 类案检索，裁判规则提炼，胜诉率分析 |
| Skill | `legal-document-drafting` | 法律文书起草，事实陈述，法律适用，论证逻辑 |
| Skill | `trial-preparation` | 庭审准备，争议焦点归纳，质证提纲，辩论要点 |

---

### 组合包 LAW-03：合规与风控包

**适用场景：** 企业合规部门、风控部门、合规咨询

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-compliance-check` | 合规检查清单，合规风险识别，整改建议 |
| 插件 | `dsh-plugin-policy-management` | 公司政策制度管理，制度起草，版本控制 |
| 插件 | `dsh-plugin-risk-assessment` | 风险评估矩阵，风险等级评定，应对措施 |
| MCP | `regulation-mcp` | 监管法规数据库，新规监测，合规要求提取 |
| MCP | `sanction-list-mcp` | 制裁名单筛查，反洗钱名单查询 |
| MCP | `gdpr-mcp` | GDPR 数据保护合规检查 |
| Skill | `compliance-audit` | 合规审计执行，问题发现，整改跟踪，审计报告 |
| Skill | `policy-drafting` | 合规政策起草，制度体系设计，流程优化 |
| Skill | `regulatory-monitoring` | 监管动态跟踪，新规影响评估，合规更新建议 |

---

## 3. 金融财经行业

### 组合包 FIN-01：投资研究与分析包

**适用场景：** 券商、基金、投资机构的投研工作

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-stock-analysis` | 股票分析，行情数据，技术指标，基本面分析 |
| 插件 | `dsh-plugin-financial-data` | 财务数据获取，三大报表，财务比率计算 |
| 插件 | `dsh-plugin-research-report` | 研报管理，研报解析，观点提取 |
| MCP | `yfinance-mcp` | Yahoo Finance 行情数据获取 |
| MCP | `sec-edgar-mcp` | SEC EDGAR 财报检索，10-K/10-Q 读取 |
| MCP | `alpha-vantage-mcp` | Alpha Vantage 金融数据 API |
| MCP | `bloomberg-mcp` | Bloomberg 终端数据接口 |
| Skill | `equity-research` | 个股深度研究，估值建模，投资建议，目标价 |
| Skill | `financial-statement-analysis` | 财务报表分析，盈利质量，偿债能力，运营效率 |
| Skill | `industry-research` | 行业研究，竞争格局，产业链分析，趋势预测 |

---

### 组合包 FIN-02：银行与信贷包

**适用场景：** 银行、信贷机构、小贷公司的信贷业务

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-credit-assessment` | 信用评估，征信报告解析，授信额度测算 |
| 插件 | `dsh-plugin-loan-management` | 贷款管理，还款计划，逾期跟踪，催收管理 |
| 插件 | `dsh-plugin-risk-pricing` | 风险定价，利率测算，违约概率模型 |
| MCP | `credit-bureau-mcp` | 征信机构数据接口，个人/企业征信查询 |
| MCP | `bank-statement-mcp` | 银行流水解析，收支分析，现金流评估 |
| MCP | `business-registry-mcp` | 工商登记信息查询，企业股权结构，司法风险 |
| Skill | `credit-application-review` | 信贷申请审查，资料核验，风险点识别，审批意见 |
| Skill | `loan-document-generation` | 贷款合同生成，借款协议，担保合同，还款计划 |
| Skill | `collection-strategy` | 催收策略制定，话术生成，催收记录，回款跟踪 |

---

### 组合包 FIN-03：保险业务包

**适用场景：** 保险公司、保险代理、保险经纪

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-insurance-product` | 保险产品管理，条款解析，费率计算，方案对比 |
| 插件 | `dsh-plugin-policy-management` | 保单管理，承保流程，批改理赔，续保提醒 |
| 插件 | `dsh-plugin-claims` | 理赔管理，报案登记，定损核赔，赔付计算 |
| MCP | `insurance-database-mcp` | 保险产品数据库，条款库，费率表查询 |
| MCP | `medical-claim-mcp` | 医保对接，医疗理赔数据接口 |
| Skill | `insurance-scheme-design` | 保险方案设计，需求分析，产品组合，保费测算 |
| Skill | `underwriting-assessment` | 核保评估，风险分类，承保决策，条件设定 |
| Skill | `claims-processing` | 理赔处理，责任认定，损失核定，赔付计算，纠纷处理 |

---

## 4. 教育培训行业

### 组合包 EDU-01：课程设计与教学包

**适用场景：** 学校、培训机构、在线教育的课程开发与教学

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-course-design` | 课程设计，大纲生成，教案编写，课件制作 |
| 插件 | `dsh-plugin-question-bank` | 题库管理，题目生成，组卷系统，自动批改 |
| 插件 | `dsh-plugin-knowledge-graph` | 知识图谱构建，知识点关联，学习路径规划 |
| MCP | `canvas-lms-mcp` | Canvas LMS 学习管理系统集成 |
| MCP | `moodle-mcp` | Moodle 学习平台接口 |
| MCP | `youtube-mcp` | YouTube 视频搜索，教育视频获取 |
| Skill | `lesson-plan-generation` | 教案生成，教学目标，教学过程，板书设计，作业布置 |
| Skill | `question-generation` | 题目自动生成，难度分级，答案解析，知识点标注 |
| Skill | `knowledge-explanation` | 知识点讲解，类比举例，易错点提示，拓展延伸 |

---

### 组合包 EDU-02：学生管理与辅导包

**适用场景：** 学校班主任、培训机构教务、一对一辅导

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-student-management` | 学生档案管理，成绩跟踪，出勤记录，表现评价 |
| 插件 | `dsh-plugin-parent-communication` | 家长沟通，家校联系，成长报告，通知推送 |
| 插件 | `dsh-plugin-tutoring` | 一对一辅导，答疑解惑，学习诊断，个性化指导 |
| MCP | `student-info-system-mcp` | 学生信息系统接口，学籍管理 |
| MCP | `gradebook-mcp` | 成绩册管理，分数录入，统计分析 |
| Skill | `student-assessment` | 学情分析，学习诊断，薄弱环节识别，提升建议 |
| Skill | `parent-report` | 家长报告生成，学习情况总结，进步亮点，改进建议 |
| Skill | `tutoring-session` | 辅导课准备，知识点讲解，例题设计，课后练习，效果评估 |

---

### 组合包 EDU-03：升学规划与志愿填报包

**适用场景：** 升学规划机构、高中学校、学生家长

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-college-planning` | 升学规划，目标设定，时间线管理，背景提升 |
| 插件 | `dsh-plugin-volunteer-filling` | 志愿填报，院校推荐，专业选择，录取概率 |
| 插件 | `dsh-plugin-exam-prep` | 考试备考，复习计划，真题解析，模考分析 |
| MCP | `college-database-mcp` | 院校数据库，招生计划，历年分数线，专业介绍 |
| MCP | `gaokao-mcp` | 高考数据接口，一分一段表，录取数据 |
| Skill | `academic-planning` | 学业规划，选科建议，竞赛规划，课外活动设计 |
| Skill | `volunteer-strategy` | 志愿填报策略，冲稳保梯度设计，院校专业匹配，风险评估 |
| Skill | `exam-preparation-plan` | 备考计划制定，知识点梳理，薄弱环节突破，时间管理 |

---

## 5. 电商零售行业

### 组合包 EC-01：商品运营与 listing 优化包

**适用场景：** 电商卖家、运营团队的商品管理与优化

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-product-management` | 商品管理，SKU 管理，库存同步，价格调整 |
| 插件 | `dsh-plugin-listing-optimization` | Listing 优化，标题生成，关键词布局，详情页设计 |
| 插件 | `dsh-plugin-keyword-research` | 关键词研究，搜索量分析，竞争度评估，选词建议 |
| MCP | `amazon-sp-mcp` | Amazon SP-API 卖家平台接口 |
| MCP | `shopify-mcp` | Shopify 电商平台接口 |
| MCP | `taobao-mcp` | 淘宝/天猫开放平台接口 |
| MCP | `jd-mcp` | 京东开放平台接口 |
| Skill | `listing-copywriting` | 商品文案撰写，卖点提炼，标题优化，描述生成 |
| Skill | `keyword-strategy` | 关键词策略制定，词库构建，排名跟踪，SEO 优化 |
| Skill | `product-photography-guidance` | 商品拍摄指导，场景设计，修图建议，A+ 内容规划 |

---

### 组合包 EC-02：客服与售后包

**适用场景：** 电商客服团队、售后部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-customer-service` | 客服管理，话术库，快捷回复，会话记录 |
| 插件 | `dsh-plugin-after-sales` | 售后管理，退换货处理，退款审核，纠纷处理 |
| 插件 | `dsh-plugin-review-management` | 评价管理，好评维护，差评处理，追评引导 |
| MCP | `customer-service-mcp` | 客服系统接口，消息收发，工单管理 |
| MCP | `order-management-mcp` | 订单系统接口，订单查询，状态更新 |
| Skill | `customer-inquiry-response` | 客户咨询回复，产品答疑，物流查询，优惠说明 |
| Skill | `complaint-handling` | 投诉处理，情绪安抚，问题诊断，解决方案，补偿方案 |
| Skill | `review-response` | 评价回复撰写，差评挽回，好评感谢，品牌形象维护 |

---

### 组合包 EC-03：营销与推广包

**适用场景：** 电商营销团队、推广部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-ad-management` | 广告管理，投放计划，出价优化，效果分析 |
| 插件 | `dsh-plugin-promotion` | 促销活动策划，优惠券设计，满减规则，活动执行 |
| 插件 | `dsh-plugin-livestream` | 直播带货，脚本撰写，话术设计，数据复盘 |
| MCP | `facebook-ads-mcp` | Facebook 广告管理接口 |
| MCP | `google-ads-mcp` | Google Ads 广告接口 |
| MCP | `tiktok-ads-mcp` | TikTok 广告接口 |
| Skill | `ad-copywriting` | 广告文案撰写，创意生成，A/B 测试方案，落地页优化 |
| Skill | `promotion-planning` | 促销活动策划，节奏安排，利益点设计，预算分配，效果预估 |
| Skill | `livestream-script` | 直播脚本撰写，产品讲解，互动设计，逼单话术，复盘分析 |

---

## 6. 制造业 / 工业

### 组合包 MFG-01：生产管理与排程包

**适用场景：** 制造企业生产部门、计划部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-production-planning` | 生产计划，排程优化，产能平衡，交期管理 |
| 插件 | `dsh-plugin-work-order` | 工单管理，生产指令，进度跟踪，报工记录 |
| 插件 | `dsh-plugin-equipment-management` | 设备管理，台账维护，保养计划，故障记录 |
| MCP | `erp-mcp` | ERP 系统接口（SAP/Oracle/用友/金蝶） |
| MCP | `mes-mcp` | MES 制造执行系统接口 |
| MCP | `iot-mcp` | IoT 设备数据采集，传感器数据接口 |
| Skill | `production-scheduling` | 生产排程优化，工序安排，资源分配，瓶颈识别 |
| Skill | `work-order-management` | 工单全流程管理，下达，执行，报工，完工，异常处理 |
| Skill | `oee-analysis` | 设备综合效率 OEE 分析，停机原因，稼动率提升方案 |

---

### 组合包 MFG-02：质量管理与控制包

**适用场景：** 制造企业质量部门、QC/QA 团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-quality-control` | 质量控制，检验标准，抽样方案，判定规则 |
| 插件 | `dsh-plugin-nonconformance` | 不合格品管理，8D 报告，纠正预防措施 |
| 插件 | `dsh-plugin-quality-certification` | 质量认证管理，ISO 体系，审核准备，证书维护 |
| MCP | `qms-mcp` | QMS 质量管理系统接口 |
| MCP | `lab-information-mcp` | LIMS 实验室信息管理系统接口 |
| Skill | `quality-inspection` | 质量检验执行，检验记录，缺陷分类，判定结论 |
| Skill | `problem-solving-8d` | 8D 问题解决，根因分析，纠正措施，效果验证，预防措施 |
| Skill | `quality-system-audit` | 质量体系审核，内审执行，不符合项，整改跟踪，管理评审 |

---

### 组合包 MFG-03：供应链与采购包

**适用场景：** 制造企业采购部门、供应链管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-procurement` | 采购管理，供应商管理，询价比价，采购订单 |
| 插件 | `dsh-plugin-inventory` | 库存管理，库存预警，安全库存，盘点管理 |
| 插件 | `dsh-plugin-supplier-evaluation` | 供应商评估，绩效考核，分级管理，淘汰机制 |
| MCP | `srm-mcp` | SRM 供应商关系管理系统接口 |
| MCP | `wms-mcp` | WMS 仓库管理系统接口 |
| MCP | `logistics-mcp` | 物流跟踪接口，运输状态查询 |
| Skill | `procurement-strategy` | 采购策略制定，品类管理，成本分析，谈判方案，合同条款 |
| Skill | `inventory-optimization` | 库存优化，ABC 分类，安全库存计算，周转率提升，呆滞处理 |
| Skill | `supplier-development` | 供应商开发，准入评估，能力提升，绩效改进，战略合作 |

---

## 7. 房地产 / 建筑

### 组合包 REAL-01：房产销售与经纪包

**适用场景：** 房产中介、售楼处、房产经纪公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-property-management` | 房源管理，房源录入，图片管理，房源匹配 |
| 插件 | `dsh-plugin-customer-followup` | 客户管理，需求分析，带看记录，跟进跟踪 |
| 插件 | `dsh-plugin-transaction` | 交易管理，合同签订，贷款办理，过户流程 |
| MCP | `mls-mcp` | MLS 多重上市服务系统接口（房产 listing） |
| MCP | `property-tax-mcp` | 房产税查询，评估价值接口 |
| MCP | `mortgage-calculator-mcp` | 房贷计算接口，利率查询，还款计划 |
| Skill | `property-listing-writing` | 房源描述撰写，卖点提炼，标题优化，文案生成 |
| Skill | `customer-needs-analysis` | 客户需求分析，购房预算，区域偏好，户型需求，匹配推荐 |
| Skill | `transaction-coordination` | 交易流程协调，合同准备，贷款协助，过户安排，交房验房 |

---

### 组合包 REAL-02：建筑工程管理包

**适用场景：** 建筑公司、工程管理、施工项目部

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-project-management` | 工程项目管理，进度计划，里程碑，关键路径 |
| 插件 | `dsh-plugin-construction-drawing` | 施工图纸管理，图纸会审，设计变更，图纸版本 |
| 插件 | `dsh-plugin-safety-management` | 安全管理，安全检查，隐患排查，事故处理 |
| MCP | `bim-mcp` | BIM 建筑信息模型接口 |
| MCP | `project-management-mcp` | 工程项目管理系统接口（Primavera/MS Project） |
| MCP | `construction-cost-mcp` | 工程造价接口，定额库，清单计价 |
| Skill | `construction-schedule` | 施工进度计划编制，工序安排，资源配置，进度跟踪，偏差分析 |
| Skill | `quality-inspection-construction` | 施工质量检查，分部分项验收，隐蔽工程，质量评定，资料整理 |
| Skill | `safety-inspection` | 安全检查执行，隐患识别，风险分级，整改通知，复查验收，教育交底 |

---

### 组合包 REAL-03：物业管理包

**适用场景：** 物业公司、物业项目部、社区管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-property-service` | 物业服务管理，报修处理，投诉建议，满意度调查 |
| 插件 | `dsh-plugin-fee-collection` | 收费管理，物业费，水电费，停车费，催缴通知 |
| 插件 | `dsh-plugin-facility-maintenance` | 设施设备维护，保养计划，维修记录，备件管理 |
| MCP | `property-management-system-mcp` | 物业管理系统接口 |
| MCP | `iot-smart-building-mcp` | 智慧楼宇 IoT 接口，设备状态监控 |
| Skill | `resident-communication` | 业主沟通，通知公告，活动组织，关系维护，纠纷调解 |
| Skill | `maintenance-work-order` | 维修工单处理，派单调度，进度跟踪，完工验收，满意度回访 |
| Skill | `property-financial-report` | 物业财务报告，收支分析，预算执行，成本控制，经营建议 |

---

## 8. 媒体 / 内容创作

### 组合包 MEDIA-01：短视频创作包

**适用场景：** 短视频创作者、MCN 机构、自媒体团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-video-script` | 视频脚本撰写，分镜设计，台词生成，节奏规划 |
| 插件 | `dsh-plugin-video-editing` | 视频剪辑辅助，剪辑建议，转场设计，字幕生成 |
| 插件 | `dsh-plugin-content-calendar` | 内容日历，选题规划，发布排期，热点跟踪 |
| MCP | `tiktok-mcp` | TikTok/抖音数据接口，视频管理，评论互动 |
| MCP | `youtube-mcp` | YouTube 视频管理，数据分析，评论回复 |
| MCP | `bilibili-mcp` | B站视频接口，弹幕管理，数据统计 |
| Skill | `short-video-script` | 短视频脚本创作，黄金3秒，钩子设计，剧情结构，结尾引导 |
| Skill | `video-title-thumbnail` | 视频标题优化，封面设计建议，关键词布局，点击率提升 |
| Skill | `content-strategy` | 内容策略制定，账号定位，选题库构建，爆款分析，增长方案 |

---

### 组合包 MEDIA-02：图文内容创作包

**适用场景：** 公众号、小红书、知乎、博客等图文创作者

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-article-writing` | 文章撰写，结构规划，内容填充，风格调整 |
| 插件 | `dsh-plugin-image-generation` | AI 图片生成，封面设计，配图制作，风格统一 |
| 插件 | `dsh-plugin-seo-optimization` | SEO 优化，关键词布局，meta 标签，内链策略 |
| MCP | `wordpress-mcp` | WordPress 博客接口，文章发布，媒体管理 |
| MCP | `wechat-mcp` | 微信公众号接口，图文发布，数据统计 |
| MCP | `xiaohongshu-mcp` | 小红书笔记接口，发布管理，互动数据 |
| Skill | `long-form-article` | 长文写作，深度报道，研究文章，结构设计，论证逻辑 |
| Skill | `social-media-post` | 社交媒体帖子撰写，平台适配，语气调整，话题标签，互动引导 |
| Skill | `content-repurposing` | 内容复用，一鱼多吃，跨平台改编，格式转换，系列衍生 |

---

### 组合包 MEDIA-03：品牌营销与公关包

**适用场景：** 品牌方、营销公司、公关团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-brand-strategy` | 品牌策略，定位分析，品牌故事，视觉规范 |
| 插件 | `dsh-plugin-campaign-management` | 营销活动管理，策划执行，效果追踪，复盘总结 |
| 插件 | `dsh-plugin-pr-crisis` | 公关危机管理，舆情监控，声明起草，应对方案 |
| MCP | `social-listening-mcp` | 社交聆听接口，品牌提及，舆情监控 |
| MCP | `media-database-mcp` | 媒体数据库，记者联系，发稿管理 |
| MCP | `influencer-mcp` | KOL/KOC 数据库，达人合作，效果评估 |
| Skill | `brand-storytelling` | 品牌故事创作，价值主张，品牌人格，传播话术，内容体系 |
| Skill | `marketing-campaign` | 营销活动策划，目标设定，创意概念，执行方案，预算分配，KPI |
| Skill | `crisis-communication` | 危机公关应对，舆情分析，声明撰写，媒体沟通，善后处理，品牌修复 |

---

## 9. 政府 / 公共服务

### 组合包 GOV-01：政务办公与公文包

**适用场景：** 政府机关、事业单位、公共服务部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-official-document` | 公文写作，格式规范，文种选择，行文规则 |
| 插件 | `dsh-plugin-meeting-management` | 会议管理，会议通知，纪要整理，决议跟踪 |
| 插件 | `dsh-plugin-government-affairs` | 政务管理，政策研究，文件流转，审批办理 |
| MCP | `government-portal-mcp` | 政府门户网站接口，信息发布，办事指南 |
| MCP | `oa-system-mcp` | OA 办公系统接口，流程审批，公文流转 |
| Skill | `official-document-drafting` | 公文起草，通知/报告/请示/批复/函，格式规范，语言风格，审核要点 |
| Skill | `policy-research` | 政策研究，文件解读，背景分析，影响评估，实施建议 |
| Skill | `meeting-minutes` | 会议纪要整理，议题记录，发言摘要，决议事项，待办分解 |

---

### 组合包 GOV-02：公共服务与民生包

**适用场景：** 街道社区、政务服务中心、民生服务部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-resident-service` | 居民服务，办事指南，政策咨询，预约办理 |
| 插件 | `dsh-plugin-community-management` | 社区管理，网格管理，居民档案，活动组织 |
| 插件 | `dsh-plugin-social-assistance` | 社会救助，低保管理，临时救助，特殊群体 |
| MCP | `citizen-service-mcp` | 市民服务接口，办事预约，进度查询 |
| MCP | `social-security-mcp` | 社保接口，医保查询，养老金，失业金 |
| MCP | `housing-fund-mcp` | 公积金接口，查询提取，贷款计算 |
| Skill | `policy-explanation` | 政策解读，办事指南，条件说明，材料清单，流程指引 |
| Skill | `resident-communication` | 居民沟通，通知发布，意见收集，纠纷调解，关怀慰问 |
| Skill | `social-work-case` | 社工个案管理，需求评估，服务计划，资源链接，跟进记录，效果评估 |

---

### 组合包 GOV-03：城市管理与应急包

**适用场景：** 城管部门、应急管理局、城市运行管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-city-management` | 城市管理，市容巡查，问题上报，处置跟踪 |
| 插件 | `dsh-plugin-emergency-response` | 应急响应，预案管理，事件处置，资源调度 |
| 插件 | `dsh-plugin-risk-monitoring` | 风险监测，隐患排查，预警发布，趋势分析 |
| MCP | `iot-city-mcp` | 城市 IoT 接口，摄像头，传感器，环境监测 |
| MCP | `emergency-alert-mcp` | 应急预警接口，气象预警，地质灾害，公共安全 |
| MCP | `gis-mcp` | GIS 地理信息系统接口，地图服务，空间分析 |
| Skill | `incident-reporting` | 事件上报，情况描述，位置标注，照片记录，等级评定 |
| Skill | `emergency-plan-execution` | 应急预案执行，响应启动，指挥协调，物资调配，信息发布，善后恢复 |
| Skill | `risk-assessment-public` | 公共安全风险评估，隐患识别，等级划分，管控措施，应急预案 |

---

## 10. IT / 软件开发

### 组合包 IT-01：代码开发与调试包

**适用场景：** 软件开发者、程序员、开发团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-code-editor` | 代码编辑，语法高亮，代码补全，重构支持 |
| 插件 | `dsh-plugin-debugger` | 调试辅助，断点管理，变量查看，调用栈分析 |
| 插件 | `dsh-plugin-code-review` | 代码审查，规范检查，安全扫描，优化建议 |
| MCP | `github-mcp` | GitHub 接口，代码仓库，Issue/PR，CI/CD |
| MCP | `gitlab-mcp` | GitLab 接口，仓库管理，CI/CD 流水线 |
| MCP | `context7-mcp` | 库文档检索，版本特定 API 文档注入 |
| MCP | `sequential-thinking-mcp` | 结构化思考，复杂问题分解，推理过程 |
| Skill | `code-generation` | 代码生成，需求分析，架构设计，编码实现，单元测试 |
| Skill | `bug-fixing` | Bug 修复，错误分析，根因定位，修复方案，回归验证 |
| Skill | `code-refactoring` | 代码重构，结构优化，设计模式应用，性能改进，可维护性提升 |

---

### 组合包 IT-02：DevOps 与运维包

**适用场景：** 运维工程师、DevOps 团队、SRE

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-server-management` | 服务器管理，远程连接，命令执行，状态监控 |
| 插件 | `dsh-plugin-deployment` | 部署管理，CI/CD 流水线，发布管理，回滚操作 |
| 插件 | `dsh-plugin-monitoring-alert` | 监控告警，指标采集，日志分析，告警通知 |
| MCP | `kubernetes-mcp` | Kubernetes 集群管理，Pod/Deployment/Service 操作 |
| MCP | `docker-mcp` | Docker 容器管理，镜像构建，容器操作 |
| MCP | `terraform-mcp` | Terraform 基础设施即代码，资源管理 |
| MCP | `prometheus-mcp` | Prometheus 监控指标查询 |
| MCP | `sentry-mcp` | Sentry 错误追踪，性能监控，问题管理 |
| Skill | `incident-response-it` | IT 事件响应，故障诊断，影响评估，处置方案，恢复验证，复盘总结 |
| Skill | `deployment-pipeline` | 部署流水线设计，CI/CD 配置，自动化测试，发布策略，灰度发布 |
| Skill | `infrastructure-management` | 基础设施管理，资源规划，成本优化，安全加固，高可用设计 |

---

### 组合包 IT-03：数据库与数据分析包

**适用场景：** 数据工程师、数据分析师、DBA

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-dba-tools` | DBA 工具，性能调优，索引优化，备份恢复 |
| 插件 | `dsh-plugin-data-analysis` | 数据分析，探索性分析，统计建模，洞察提取 |
| 插件 | `dsh-plugin-data-visualization` | 数据可视化，图表设计，仪表盘搭建，报告生成 |
| MCP | `postgres-mcp` | PostgreSQL 数据库操作 |
| MCP | `mysql-mcp` | MySQL 数据库操作 |
| MCP | `mongodb-mcp` | MongoDB 文档数据库操作 |
| MCP | `redis-mcp` | Redis 缓存操作 |
| MCP | `snowflake-mcp` | Snowflake 数据仓库查询 |
| Skill | `sql-optimization` | SQL 优化，执行计划分析，索引建议，查询改写，性能调优 |
| Skill | `exploratory-data-analysis` | 探索性数据分析，数据清洗，特征工程，统计检验，可视化探索 |
| Skill | `dashboard-design` | 仪表盘设计，指标体系，图表选择，布局规划，交互设计，故事线 |

---

## 11. 农业

### 组合包 AGR-01：种植管理与精准农业包

**适用场景：** 农场、种植基地、农业合作社

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-crop-management` | 作物管理，种植计划，农事记录，生长跟踪 |
| 插件 | `dsh-plugin-precision-agriculture` | 精准农业，变量施肥，灌溉控制，产量预测 |
| 插件 | `dsh-plugin-pest-disease` | 病虫害管理，识别诊断，防治方案，用药记录 |
| MCP | `weather-mcp` | 气象数据接口，天气预报，历史气象，农业气象指标 |
| MCP | `soil-mcp` | 土壤数据接口，养分检测，墒情监测，pH 值 |
| MCP | `drone-mcp` | 无人机接口，航拍图像，NDVI 植被指数，植保作业 |
| Skill | `planting-plan` | 种植计划制定，品种选择，茬口安排，播期确定，密度设计 |
| Skill | `pest-diagnosis` | 病虫害诊断，症状识别，发生规律，防治时机，药剂选择，安全用药 |
| Skill | `yield-optimization` | 产量优化，水肥管理，群体调控，逆境应对，收获时机，品质提升 |

---

### 组合包 AGR-02：农产品销售与溯源包

**适用场景：** 农产品销售、农业电商、食品企业

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-agri-product-sales` | 农产品销售，订单管理，客户管理，渠道拓展 |
| 插件 | `dsh-plugin-traceability` | 溯源管理，生产记录，质量检测，溯源码生成 |
| 插件 | `dsh-plugin-agri-brand` | 农业品牌，品牌故事，包装设计，营销推广 |
| MCP | `ecommerce-platform-mcp` | 电商平台接口，农产品上架，订单同步 |
| MCP | `food-safety-mcp` | 食品安全接口，检测报告，合格证，追溯查询 |
| Skill | `agri-product-marketing` | 农产品营销，卖点提炼，产地故事，内容创作，渠道选择 |
| Skill | `traceability-system` | 溯源体系建设，信息采集，区块链存证，溯源查询，消费者信任 |
| Skill | `agri-supply-chain` | 农产品供应链，采收管理，冷链物流，仓储保鲜，损耗控制，品质保障 |

---

### 组合包 AGR-03：畜牧养殖包

**适用场景：** 养殖场、畜牧企业、养殖合作社

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-livestock-management` | 畜牧管理，个体档案，繁殖记录，生长性能 |
| 插件 | `dsh-plugin-feed-management` | 饲料管理，配方设计，饲喂记录，成本核算 |
| 插件 | `dsh-plugin-animal-health` | 动物健康，免疫程序，疾病防治，用药记录 |
| MCP | `iot-livestock-mcp` | 畜牧 IoT 接口，耳标数据，环境监测，体重采集 |
| MCP | `veterinary-drug-mcp` | 兽药数据库，说明书，休药期，相互作用 |
| Skill | `breeding-management` | 育种管理，选种选配，繁殖计划，妊娠诊断，产仔护理，断奶管理 |
| Skill | `feed-formulation` | 饲料配方设计，营养需求，原料选择，成本优化，阶段饲喂，效果评估 |
| Skill | `biosecurity` | 生物安全，消毒程序，免疫规划，疫病监测，应急处置，隔离检疫 |

---

## 12. 物流 / 供应链

### 组合包 LOG-01：运输与配送包

**适用场景：** 物流公司、运输企业、配送团队

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-transport-planning` | 运输计划，路线规划，车辆调度，配载优化 |
| 插件 | `dsh-plugin-delivery-management` | 配送管理，订单分配，路径优化，签收管理 |
| 插件 | `dsh-plugin-fleet-management` | 车队管理，车辆档案，司机管理，油耗监控 |
| MCP | `gps-tracking-mcp` | GPS 定位接口，车辆追踪，轨迹回放，电子围栏 |
| MCP | `map-navigation-mcp` | 地图导航接口，路径规划，实时路况，地理编码 |
| MCP | `express-mcp` | 快递接口，运单查询，物流轨迹，电子面单 |
| Skill | `route-optimization` | 路径优化，VRP 问题求解，时间窗约束，车辆容量，成本最小化 |
| Skill | `delivery-execution` | 配送执行，订单分拣，装车清单，配送顺序，异常处理，客户签收 |
| Skill | `fleet-efficiency` | 车队效率分析，油耗管理，维保计划，司机绩效，安全管理，成本控制 |

---

### 组合包 LOG-02：仓储与库存包

**适用场景：** 仓储企业、物流中心、仓库管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-warehouse-layout` | 仓库布局，库位规划，动线设计，容量优化 |
| 插件 | `dsh-plugin-inbound-outbound` | 出入库管理，收货质检，上架拣货，盘点调拨 |
| 插件 | `dsh-plugin-stock-control` | 库存控制，安全库存，补货策略，周转率 |
| MCP | `wms-mcp` | WMS 仓库管理系统接口 |
| MCP | `barcode-rfid-mcp` | 条码/RFID 接口，扫码操作，标签打印 |
| MCP | `automated-storage-mcp` | 自动化立体仓库接口，AGV，堆垛机 |
| Skill | `warehouse-operations` | 仓储作业优化，收货流程，存储策略，拣货路径，发货流程，退货处理 |
| Skill | `inventory-accuracy` | 库存准确率提升，循环盘点，差异处理，系统调整，防错机制，绩效考核 |
| Skill | `space-utilization` | 空间利用率提升，库位优化，ABC 分类，动态调整，扩容方案，成本分析 |

---

### 组合包 LOG-03：供应链协同包

**适用场景：** 供应链管理、采购物流、供应链协同

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-supply-chain-planning` | 供应链计划，需求预测，库存策略，协同计划 |
| 插件 | `dsh-plugin-procurement-logistics` | 采购物流，供应商交付，在途跟踪，入库协同 |
| 插件 | `dsh-plugin-supply-chain-risk` | 供应链风险，风险识别，中断预警，应急方案 |
| MCP | `scm-system-mcp` | SCM 供应链管理系统接口 |
| MCP | `edi-mcp` | EDI 电子数据交换接口，订单/发票/发货通知 |
| MCP | `trade-customs-mcp` | 国际贸易海关接口，报关单，关税，原产地 |
| Skill | `demand-forecasting` | 需求预测，历史分析，季节性，趋势判断，促销影响，预测准确率 |
| Skill | `supply-chain-collaboration` | 供应链协同，信息共享，联合计划，VMI 管理，CPFR 流程，绩效评估 |
| Skill | `supply-chain-resilience` | 供应链韧性，风险地图，多源采购，安全库存，应急物流，业务连续性 |

---

## 13. 旅游 / 酒店

### 组合包 TRAV-01：旅行社与行程规划包

**适用场景：** 旅行社、旅游顾问、定制游公司

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-itinerary-design` | 行程设计，路线规划，景点安排，时间管理 |
| 插件 | `dsh-plugin-tour-product` | 旅游产品管理，产品打包，定价策略，库存管理 |
| 插件 | `dsh-plugin-tour-guide` | 导游辅助，讲解词，景点介绍，应急处理 |
| MCP | `flight-mcp` | 机票接口，航班查询，价格对比，预订管理 |
| MCP | `hotel-booking-mcp` | 酒店预订接口，房态查询，价格比较，预订确认 |
| MCP | `attraction-ticket-mcp` | 景点门票接口，票务查询，在线预订，核销管理 |
| MCP | `travel-review-mcp` | 旅游评价接口，景点评分，游客评论，攻略推荐 |
| Skill | `custom-itinerary` | 定制行程设计，需求分析，预算控制，兴趣匹配，节奏安排，特色体验 |
| Skill | `tour-costing` | 旅游成本核算，分项报价，利润分析，价格策略，成本控制，报价单生成 |
| Skill | `tour-guide-script` | 导游词撰写，景点讲解，历史文化，故事传说，互动设计，应急话术 |

---

### 组合包 TRAV-02：酒店运营与管理包

**适用场景：** 酒店、民宿、度假村的运营管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-hotel-front-desk` | 前台管理，入住退房，房态管理，客人服务 |
| 插件 | `dsh-plugin-revenue-management` | 收益管理，动态定价，房量控制，渠道管理 |
| 插件 | `dsh-plugin-hotel-housekeeping` | 客房管理，清洁计划，查房标准，布草管理 |
| MCP | `pms-mcp` | PMS 酒店管理系统接口 |
| MCP | `ota-mcp` | OTA 渠道接口（携程/美团/Booking/Agoda） |
| MCP | `channel-manager-mcp` | 渠道管理器接口，房态同步，价格同步 |
| Skill | `front-desk-operations` | 前台操作，入住办理，退房结算，投诉处理，升级销售，客人关系 |
| Skill | `dynamic-pricing` | 动态定价策略，需求预测，竞争分析，价格调整，库存分配，收益最大化 |
| Skill | `guest-experience` | 客人体验提升，服务标准，个性化服务，惊喜设计，满意度调查，差评挽回 |

---

### 组合包 TRAV-03：旅游营销与获客包

**适用场景：** 旅游营销、目的地推广、旅游电商

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-travel-marketing` | 旅游营销，内容创作，活动策划，获客转化 |
| 插件 | `dsh-plugin-destination-promotion` | 目的地推广，形象塑造，资源整合，品牌传播 |
| 插件 | `dsh-plugin-travel-crm` | 旅游 CRM，客户管理，会员体系，复购营销 |
| MCP | `social-media-travel-mcp` | 社交媒体旅游接口，小红书/抖音/马蜂窝 |
| MCP | `travel-kol-mcp` | 旅游 KOL 数据库，达人合作，内容种草 |
| Skill | `travel-content-marketing` | 旅游内容营销，攻略撰写，游记创作，短视频脚本，图片规划 |
| Skill | `travel-campaign` | 旅游活动策划，主题设计，优惠方案，传播路径，效果追踪，ROI 分析 |
| Skill | `travel-member-operations` | 旅游会员运营，会员分层，权益设计，积分体系，召回策略，LTV 提升 |

---

## 14. 餐饮行业

### 组合包 FOOD-01：餐厅运营与管理包

**适用场景：** 餐厅、快餐店、餐饮连锁的运营管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-restaurant-pos` | 收银管理，点餐下单，结账支付，交班对账 |
| 插件 | `dsh-plugin-kitchen-management` | 厨房管理，出品管理，出餐节奏，菜品标准 |
| 插件 | `dsh-plugin-restaurant-service` | 服务管理，服务标准，桌台管理，客人体验 |
| MCP | `pos-system-mcp` | POS 收银系统接口 |
| MCP | `restaurant-order-mcp` | 点餐系统接口，外卖平台对接 |
| MCP | `kitchen-display-mcp` | KDS 厨房显示系统接口 |
| Skill | `restaurant-operations` | 餐厅运营管理，排班计划，流程优化，成本控制，质量标准，现场管理 |
| Skill | `menu-engineering` | 菜单工程，菜品分析，明星/瘦狗/问题/耕牛分类，定价策略，菜单优化 |
| Skill | `customer-service-restaurant` | 餐饮服务，迎客入座，点餐推荐，上菜服务，投诉处理，送客关怀 |

---

### 组合包 FOOD-02：菜品研发与供应链包

**适用场景：** 餐饮研发、中央厨房、食材采购

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-menu-development` | 菜品研发，菜谱管理，标准配方，成本核算 |
| 插件 | `dsh-plugin-central-kitchen` | 中央厨房，生产计划，半成品加工，配送管理 |
| 插件 | `dsh-plugin-food-purchasing` | 食材采购，供应商管理，询价比价，库存管理 |
| MCP | `food-cost-mcp` | 食材成本数据库，价格行情，供应商报价 |
| MCP | `food-safety-mcp` | 食品安全接口，检测报告，溯源查询，证照管理 |
| MCP | `recipe-database-mcp` | 菜谱数据库，菜品配方，烹饪方法，营养成分 |
| Skill | `recipe-development` | 菜品研发，口味设计，配方调试，标准化，成本控制，上市测试 |
| Skill | `food-cost-control` | 食材成本控制，标准成本，实际成本，差异分析，损耗管理，采购优化 |
| Skill | `central-kitchen-production` | 中央厨房生产，产能规划，生产排程，质量控制，冷链配送，门店对接 |

---

### 组合包 FOOD-03：餐饮营销与外卖包

**适用场景：** 餐饮营销、外卖运营、会员管理

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-food-marketing` | 餐饮营销，活动策划，优惠券，会员营销 |
| 插件 | `dsh-plugin-delivery-operations` | 外卖运营，平台管理，菜品优化，评价管理 |
| 插件 | `dsh-plugin-restaurant-crm` | 餐饮 CRM，会员管理，储值卡，积分体系 |
| MCP | `meituan-mcp` | 美团外卖接口，店铺管理，订单处理，评价回复 |
| MCP | `eleme-mcp` | 饿了么外卖接口，店铺运营，订单管理，数据统计 |
| MCP | `dianping-mcp` | 大众点评接口，评价管理，团购套餐，数据统计 |
| Skill | `restaurant-promotion` | 餐饮促销活动，节日营销，新品推广，套餐设计，满减策略，复购提升 |
| Skill | `delivery-store-optimization` | 外卖店铺优化，菜单结构，菜品图片，标题关键词，评分维护，转化率提升 |
| Skill | `restaurant-member-operations` | 餐饮会员运营，会员分层，权益设计，储值营销，生日关怀，私域运营 |

---

## 15. 人力资源

### 组合包 HR-01：招聘与人才获取包

**适用场景：** 企业 HR、招聘专员、猎头顾问

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-recruitment` | 招聘管理，职位发布，简历筛选，面试安排 |
| 插件 | `dsh-plugin-talent-search` | 人才搜索，候选人画像，能力评估，匹配推荐 |
| 插件 | `dsh-plugin-interview` | 面试管理，面试题库，评估表，面试反馈 |
| MCP | `linkedin-mcp` | LinkedIn 接口，人才搜索，职位发布 |
| MCP | `boss-zhipin-mcp` | BOSS直聘接口，候选人沟通，职位管理 |
| MCP | `ats-mcp` | ATS 招聘管理系统接口 |
| Skill | `job-description-writing` | 职位描述撰写，岗位职责，任职要求，薪酬范围，雇主品牌，吸引点 |
| Skill | `resume-screening` | 简历筛选，关键词匹配，经历分析，能力评估，候选人分级，面试推荐 |
| Skill | `interview-question-design` | 面试题设计，结构化面试，行为面试，技术面试，评估标准，打分表 |

---

### 组合包 HR-02：员工管理与发展包

**适用场景：** 企业 HR、人事部门、培训部门

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-employee-management` | 员工管理，人事档案，入转调离，合同管理 |
| 插件 | `dsh-plugin-performance` | 绩效管理，目标设定，考核评估，反馈辅导 |
| 插件 | `dsh-plugin-training-development` | 培训发展，培训计划，课程管理，效果评估 |
| MCP | `hris-mcp` | HRIS 人力资源信息系统接口 |
| MCP | `performance-system-mcp` | 绩效管理系统接口 |
| MCP | `lms-mcp` | LMS 学习管理系统接口 |
| Skill | `employee-onboarding` | 员工入职，入职准备，欢迎流程，培训计划，导师分配，试用期管理，融入跟踪 |
| Skill | `performance-management` | 绩效管理，OKR/KPI 设定，过程跟踪，绩效评估，反馈面谈，改进计划，结果应用 |
| Skill | `training-needs-analysis` | 培训需求分析，能力差距，发展计划，课程设计，培训实施，效果评估，ROI 分析 |

---

### 组合包 HR-03：薪酬福利与员工关系包

**适用场景：** 企业薪酬福利、员工关系、工会工作

| 类型 | 名称 | 功能说明 |
|------|------|----------|
| 插件 | `dsh-plugin-compensation` | 薪酬管理，薪资核算，社保公积金，个税计算 |
| 插件 | `dsh-plugin-benefits` | 福利管理，福利方案，员工关怀，弹性福利 |
| 插件 | `dsh-plugin-employee-relations` | 员工关系，沟通管理，冲突调解，离职管理 |
| MCP | `payroll-mcp` | 薪资系统接口，工资核算，发放管理 |
| MCP | `social-security-mcp` | 社保公积金接口，基数申报，缴纳管理 |
| MCP | `tax-mcp` | 个税接口，专项附加扣除，汇算清缴 |
| Skill | `salary-calculation` | 薪资核算，考勤统计，绩效奖金，补贴扣款，社保个税，工资条生成 |
| Skill | `benefit-program-design` | 福利方案设计，需求调研，预算分配，项目选择，供应商管理，满意度调查 |
| Skill | `employee-relation-management` | 员工关系管理，沟通机制，满意度调查，冲突调解，违纪处理，离职面谈，法律合规 |

---

## 组合包安装配置指南

### 安装方式

#### 1. 插件安装

```bash
# 通过 DSH 插件管理器安装
dsh plugin install dsh-plugin-filesystem

# 从 GitHub 安装
dsh plugin install https://github.com/DSH-Team/dsh-plugin-filesystem

# 本地安装
dsh plugin install ./path/to/plugin
```

#### 2. MCP 服务器配置

在 DSH 配置文件中添加 MCP 服务器：

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/allowed/directory"]
    },
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres", "postgresql://user:pass@localhost:5432/db"]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "your-token"
      }
    }
  }
}
```

#### 3. Skill 安装

```bash
# 安装技能包
dsh skill install document-processing

# 从市场安装
dsh skill install --market dsh-skill-market

# 启用技能
dsh skill enable data-analysis-basic
```

### 组合包推荐配置

#### 初创企业最小配置

| 组合包 | 说明 |
|--------|------|
| G-01 文件与办公基础包 | 日常办公必备 |
| G-02 网络与信息检索包 | 信息搜集必备 |
| G-03 通信与协作包 | 团队协作必备 |
| 行业专属组合包 × 1 | 根据行业选择 |

#### 中型企业标准配置

| 组合包 | 说明 |
|--------|------|
| G-01 ~ G-04 全部通用包 | 完整基础能力 |
| 行业专属组合包 × 2-3 | 核心业务场景 |
| IT-01 代码开发包（如有技术团队） | 内部开发支持 |

#### 大型企业完整配置

| 组合包 | 说明 |
|--------|------|
| 全部通用基础包 | G-01 ~ G-04 |
| 行业专属组合包 × 3-5 | 覆盖主要业务线 |
| IT 运维组合包 | IT-01 ~ IT-03 |
| 人力资源组合包 | HR-01 ~ HR-03 |
| 定制开发 | 根据特殊需求定制 |

### 注意事项

1. **数据安全**：涉及敏感数据的 MCP 服务器（如数据库、文件系统）必须配置访问权限控制，限制可访问的目录和数据库
2. **API 密钥**：需要 API 密钥的 MCP 服务器（如 GitHub、Slack）请妥善保管密钥，不要提交到代码仓库
3. **性能考虑**：同时启用过多 MCP 服务器可能影响性能，建议根据实际需求启用，不需要的及时禁用
4. **版本兼容**：不同版本的 MCP 服务器可能存在接口差异，建议使用经过测试的稳定版本
5. **网络要求**：部分 MCP 服务器需要访问外部 API，请确保网络连通性，必要时配置代理

---

**文档结束**

> 本文档为 DSH 各行业插件/MCP/Skill 组合包参考目录，实际使用时请根据具体业务需求选择和定制。组合包列表将持续更新，建议定期查看最新版本。
