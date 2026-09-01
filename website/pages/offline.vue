<template>
  <div class="offline-page">
    <div class="bg-glow" aria-hidden="true"></div>

    <section class="page-head">
      <div class="container">
        <h1>离线部署</h1>
        <p>把插件组合包带到无外网的环境：导出离线包 → 离线机导入，两步完成</p>
      </div>
    </section>

    <section class="offline-body">
      <div class="container">
        <div class="hero-actions card">
          <div class="online-info">
            <div class="online-icon">📦</div>
            <div>
              <h3>需要先在联网机器上打包</h3>
              <p>桌面客户端内置「快照与离线打包」，扫描已安装插件并生成自包含离线包（含全部插件文件与依赖清单），无需手动收集目录</p>
            </div>
          </div>
          <div class="hero-btns">
            <a href="dshupdater://offline" class="btn btn-primary">打开离线打包</a>
            <a href="dshupdater://open" class="btn btn-outline">打开工具</a>
          </div>
        </div>

        <div class="section-title">
          <h2>三步完成离线部署</h2>
          <p>全程不需要离线机访问外网</p>
        </div>

        <div class="steps-grid">
          <div class="step-card card">
            <div class="step-no">1</div>
            <h3>联网机：生成离线包</h3>
            <p>在装有客户端的联网机器上，进入「快照与离线打包」→「离线部署」→ 选择要带走的插件 → 生成 <code>.zip</code> 离线包（含依赖清单与校验信息）</p>
          </div>
          <div class="step-card card">
            <div class="step-no">2</div>
            <h3>物理转移</h3>
            <p>通过 U 盘 / 内网共享等方式把离线包复制到目标机器。包内自包含，转移过程零外网依赖</p>
          </div>
          <div class="step-card card">
            <div class="step-no">3</div>
            <h3>离线机：一键导入</h3>
            <p>目标机器安装客户端后，打开「快照与离线打包」→「导入离线包」→ 选择 zip 即可还原全部插件，逐项校验完整性</p>
          </div>
        </div>

        <div class="note-card card">
          <h3>适用场景</h3>
          <ul>
            <li>企业内网 / 隔离区机器需要统一的插件环境</li>
            <li>一次打包，多台同构机器批量分发</li>
            <li>外网受限环境下的插件升级接力（联网机代拉新版本 → 离线包中转）</li>
          </ul>
          <p class="note-foot">需要先安装桌面客户端（v1.11.0 及以上支持从浏览器直接唤起离线打包）。</p>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
useHead({ title: '离线部署 - DSH 插件升级管理工具' })
</script>

<style scoped>
.offline-page { min-height: 100vh; position: relative; }
.page-head { padding: 64px 0 28px; text-align: center; }
.page-head h1 { font-size: 34px; margin-bottom: 8px; }
.page-head p { color: var(--text-2); }
.offline-body { padding-bottom: 80px; }
.hero-actions { display: flex; align-items: center; justify-content: space-between; gap: 20px; flex-wrap: wrap; padding: 28px 32px; }
.online-info { display: flex; gap: 18px; align-items: center; }
.online-icon { font-size: 40px; }
.online-info h3 { margin-bottom: 4px; }
.online-info p { color: var(--text-2); font-size: 14px; }
.hero-btns { display: flex; gap: 12px; flex-wrap: wrap; }
.btn-outline { background: var(--card); color: var(--text); border: 1px solid var(--border); }
.section-title { margin: 56px 0 24px; text-align: center; }
.section-title h2 { font-size: 26px; margin-bottom: 6px; }
.section-title p { color: var(--text-2); }
.steps-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; }
.step-card { padding: 26px 24px; }
.step-no { width: 40px; height: 40px; border-radius: 999px; display: flex; align-items: center; justify-content: center; font-weight: 700; color: #fff; background: linear-gradient(135deg, #6366f1, #38bdf8); margin-bottom: 14px; }
.step-card h3 { margin-bottom: 8px; font-size: 17px; }
.step-card p { color: var(--text-2); font-size: 14px; line-height: 1.7; }
.step-card code { background: var(--card); border: 1px solid var(--border); border-radius: 6px; padding: 1px 6px; font-size: 12.5px; }
.note-card { margin-top: 32px; padding: 26px 28px; }
.note-card h3 { margin-bottom: 10px; }
.note-card ul { padding-left: 20px; color: var(--text-2); line-height: 1.9; font-size: 14px; }
.note-foot { margin-top: 12px; color: var(--text-3); font-size: 13px; }
@media (max-width: 900px) { .steps-grid { grid-template-columns: 1fr; } }
</style>
