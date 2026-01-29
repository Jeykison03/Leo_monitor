<script setup>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import LiveChart from '../components/LiveChart.vue'

const isConnected = ref(false)
const currentBpm = ref(0)
const liveHistory = ref(new Array(60).fill(500)) // 60 data points for live graph
const avgHistory = ref(new Array(60).fill(0))

const isAlert = computed(() => currentBpm.value > 100)

const averageBpm = computed(() => {
  const validBpms = avgHistory.value.filter(v => v > 0);
  if (validBpms.length === 0) return "--";
  const sum = validBpms.reduce((a, b) => a + b, 0);
  return Math.round(sum / validBpms.length);
})

let socket = null

const connectWebSocket = () => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = window.location.host; 
    const wsUrl = `${protocol}//${host}/ws/heart_rate`;
    
    socket = new WebSocket(wsUrl);

    socket.onopen = () => {
        isConnected.value = true;
        console.log("WS Connected");
    };

    socket.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            if (data.type === 'BIO_DATA') {
                updateData(data.bpm, data.raw, data.last_beat);
            }
        } catch (e) {
            console.error("Parse Error", e);
        }
    };

    socket.onclose = () => {
        isConnected.value = false;
        setTimeout(connectWebSocket, 3000);
    };
}

const tempBuffer = ref([]);
const displayedAverage = ref("--");
let lastBeatTimestamp = null;

const updateData = (bpm, raw, lastBeat) => {
    if (bpm > 0) {
        currentBpm.value = bpm;
        
        if (lastBeat && lastBeat !== lastBeatTimestamp) {
            lastBeatTimestamp = lastBeat;
            
            tempBuffer.value.push(bpm);
            
            if (tempBuffer.value.length >= 10) {
                 const sum = tempBuffer.value.reduce((a, b) => a + b, 0);
                 const avg = Math.round(sum / tempBuffer.value.length);
                 
                 displayedAverage.value = avg;
                 
                 avgHistory.value.shift();
                 avgHistory.value.push(avg);
                 
                 tempBuffer.value = [];
            } else {
                 if (displayedAverage.value !== "--") {
                     avgHistory.value.shift();
                     avgHistory.value.push(displayedAverage.value);
                 }
            }
        } else {
             if (displayedAverage.value !== "--") {
                 avgHistory.value.shift();
                 avgHistory.value.push(displayedAverage.value);
             }
        }
    }
    
    const waveValue = raw || 500;
    liveHistory.value.shift();
    liveHistory.value.push(waveValue);
}

const minHr = ref(0);
const maxHr = ref(0);
const avgHr = ref(0);
const selectedInterval = ref("0");

const fetchStats = async () => {
    const now = new Date();
    const end = new Date(now.getTime() - (parseInt(selectedInterval.value) * 10 * 60000));
    const start = new Date(end.getTime() - (10 * 60000));
    
    const startStr = start.toISOString();
    const endStr = end.toISOString();
    
    try {
        const res = await fetch(`http://localhost:8080/api/stats?start=${startStr}&end=${endStr}`);
        const data = await res.json();
        minHr.value = data.min;
        maxHr.value = data.max;
        avgHr.value = data.avg;
    } catch (e) {
        console.error("Failed to fetch history", e);
    }
}

watch(selectedInterval, () => {
    fetchStats();
})

onMounted(() => {
    connectWebSocket();
    fetchStats();
    setInterval(fetchStats, 30000);
})

const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

const playBeep = () => {
    if (audioCtx.state === 'suspended') audioCtx.resume();
    const oscillator = audioCtx.createOscillator();
    const gainNode = audioCtx.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(audioCtx.destination);

    oscillator.type = 'sine';
    oscillator.frequency.setValueAtTime(880, audioCtx.currentTime);
    oscillator.frequency.exponentialRampToValueAtTime(440, audioCtx.currentTime + 0.1);
    
    gainNode.gain.setValueAtTime(0.1, audioCtx.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(0.01, audioCtx.currentTime + 0.1);

    oscillator.start();
    oscillator.stop(audioCtx.currentTime + 0.1);
}

let beepInterval = null;
watch(isAlert, (newVal) => {
    if (newVal) {
        playBeep();
        beepInterval = setInterval(playBeep, 1000);
    } else {
        if (beepInterval) {
            clearInterval(beepInterval);
            beepInterval = null;
        }
    }
})

onUnmounted(() => {
    if (socket) socket.close();
    if (beepInterval) clearInterval(beepInterval);
})
</script>

<template>
  <div class="dashboard" :class="{ 'alert-mode': isAlert }">
    <div class="kpi-grid">
      <div class="kpi-card glass-panel">
        <div class="kpi-label">Current Heart Rate</div>
        <div class="kpi-value" :class="{ 'text-danger': isAlert }">
          {{ currentBpm }} <span class="unit">BPM</span>
        </div>
        <div v-if="isAlert" class="alert-badge">HIGH ALERT</div>
      </div>
      
      <div class="kpi-card glass-panel">
        <div class="kpi-label"> Average </div>
        <div class="kpi-value info">
          {{ displayedAverage }} <span class="unit">BPM</span>
        </div>
      </div>
    </div>

    <div class="charts-grid">
      <div class="chart-card glass-panel">
        <div class="card-header">
           <h3>Live Heart Rate </h3>
           <div class="small-badge">Rate: {{ currentBpm }}</div>
        </div>
        <LiveChart :data="liveHistory" color="#2f81f7" />
      </div>

      <div class="chart-card glass-panel">
         <div class="card-header">
           <h3>Average Trend </h3>
           <div class="small-badge">Avg: {{ displayedAverage }}</div>
        </div>
        <LiveChart :data="avgHistory" color="#238636" />
      </div>
    </div>

    <div class="history-section glass-panel">
        <div class="history-header">
            <h3>History Analysis (10-min Intervals)</h3>
            <div class="controls">
                <select v-model="selectedInterval" class="interval-select">
                    <option value="0">Last 10 Minutes</option>
                    <option value="1">10-20 Mins Ago</option>
                    <option value="2">20-30 Mins Ago</option>
                    <option value="3">30-40 Mins Ago</option>
                    <option value="6">Last Hour</option>
                </select>
                <button @click="fetchStats" class="refresh-btn">Refresh</button>
            </div>
        </div>
        
        <div class="stats-grid">
            <div class="stat-box min">
                <div class="stat-label">Minimum</div>
                <div class="stat-value">{{ minHr }}</div>
            </div>
            <div class="stat-box avg">
                <div class="stat-label">Average</div>
                <div class="stat-value">{{ avgHr }}</div>
            </div>
            <div class="stat-box max">
                <div class="stat-label">Maximum</div>
                <div class="stat-value">{{ maxHr }}</div>
            </div>
        </div>
    </div>

    <div class="status-bar">
        Status: <span :class="isConnected ? 'text-success' : 'text-danger'">{{ isConnected ? 'Connected' : 'Disconnected' }}</span>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  padding-top: 1rem;
  transition: all 0.3s ease;
  border-radius: 20px;
}


.alert-mode {
   box-shadow: 0 0 50px rgba(248, 81, 73, 0.4) inset;
   border: 2px solid var(--danger-color);
   animation: screen-pulse 1s infinite alternate;
}

@keyframes screen-pulse {
    from { box-shadow: 0 0 20px rgba(248, 81, 73, 0.2) inset; }
    to { box-shadow: 0 0 70px rgba(248, 81, 73, 0.6) inset; }
}

.kpi-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.kpi-card {
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
}

.kpi-value {
  font-size: 3.5rem;
  font-weight: 700;
  line-height: 1;
  margin: 1rem 0;
}

.unit {
  font-size: 1rem;
  color: var(--text-secondary);
  font-weight: 400;
}

.text-danger { 
    color: var(--danger-color); 
    text-shadow: 0 0 10px rgba(248, 81, 73, 0.5);
    animation: text-pulse 0.5s infinite alternate; 
}

@keyframes text-pulse {
    from { opacity: 0.8; transform: scale(1); }
    to { opacity: 1; transform: scale(1.1); }
}

.info { color: var(--success-color); }
.text-success { color: var(--success-color); }

.charts-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

@media (max-width: 900px) {
  .charts-grid {
    grid-template-columns: 1fr;
  }
}

.chart-card {
  padding: 1.5rem;
  min-height: 300px;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.small-badge {
    background: rgba(255,255,255,0.1);
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
}

.alert-badge {
    position: absolute;
    top: 10px;
    right: 10px;
    background: var(--danger-color);
    color: white;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 800;
    box-shadow: 0 0 10px red;
    animation: flash 0.2s infinite;
}

@keyframes flash {
    0% { opacity: 1; }
    50% { opacity: 0.4; }
    100% { opacity: 1; }
}

.history-section {
    margin-top: 2rem;
    padding: 1.5rem;
    margin-bottom: 2rem;
}

.history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
    gap: 1rem;
}

.controls {
    display: flex;
    gap: 1rem;
}

.interval-select {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    padding: 0.5rem;
    border-radius: 6px;
    font-family: inherit;
}

.refresh-btn {
    background: var(--primary-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: opacity 0.2s;
}

.refresh-btn:hover {
    opacity: 0.9;
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1.5rem;
}

.stat-box {
    background: rgba(255, 255, 255, 0.03);
    border-radius: 12px;
    padding: 1.5rem;
    text-align: center;
    border: 1px solid transparent;
    transition: transform 0.2s;
}

.stat-box:hover {
    transform: translateY(-2px);
}

.stat-label {
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.stat-value {
    font-size: 2rem;
    font-weight: 700;
}

.stat-box.min .stat-value { color: #bc8cff; }
.stat-box.min { border-color: rgba(188, 140, 255, 0.2); }

.stat-box.avg .stat-value { color: #58a6ff; }
.stat-box.avg { border-color: rgba(88, 166, 255, 0.2); }

.stat-box.max .stat-value { color: #ff7b72; }
.stat-box.max { border-color: rgba(255, 123, 114, 0.2); }
</style>
