import { invoke } from "@tauri-apps/api/core";

async function loadSystemInfo() {
  const container = document.getElementById("info-container");
  container.innerHTML = '<p class="loading">加载中...</p>';

  try {
    const info = await invoke("get_system_info");

    container.innerHTML = `
      <div class="info-card">
        <span class="label">操作系统</span>
        <span class="value">${info.os}</span>
      </div>
      <div class="info-card">
        <span class="label">架构</span>
        <span class="value">${info.arch}</span>
      </div>
      <div class="info-card">
        <span class="label">Tauri 版本</span>
        <span class="value">${info.tauriVersion}</span>
      </div>
      <div class="info-card">
        <span class="label">设备名称</span>
        <span class="value">${info.deviceName || "N/A"}</span>
      </div>
    `;
  } catch (error) {
    container.innerHTML = `<p class="error">加载失败: ${error}</p>`;
  }
}

document.getElementById("refresh-btn").addEventListener("click", loadSystemInfo);
loadSystemInfo();