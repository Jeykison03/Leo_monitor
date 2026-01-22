<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import * as d3 from 'd3'

const props = defineProps({
  data: { type: Array, required: true },
  color: { type: String, default: '#58a6ff' },
  height: { type: Number, default: 200 }
})

const svgRef = ref(null)
const width = 600

const chartId = Math.random().toString(36).substr(2, 9)

onMounted(() => {
  renderChart()
})

watch(() => props.data, () => {
  updateChart()
}, { deep: true })

let svg, x, y, line, path, area;

const renderChart = () => {
  if (!svgRef.value) return;

  const margin = { top: 10, right: 0, bottom: 20, left: 30 }
  const innerWidth = width - margin.left - margin.right
  const innerHeight = props.height - margin.top - margin.bottom

  svg = d3.select(svgRef.value)
    .attr("viewBox", `0 0 ${width} ${props.height}`)
    .append("g")
    .attr("transform", `translate(${margin.left},${margin.top})`)

  svg.append("g").attr("class", "x-axis").attr("transform", `translate(0,${innerHeight})`)
  svg.append("g").attr("class", "y-axis")
  
  path = svg.append("path")
    .attr("fill", "none")
    .attr("stroke", props.color)
    .attr("stroke-width", 3)
    
  const gradient = svg.append("defs")
    .append("linearGradient")
    .attr("id", `gradient-${chartId}`)
    .attr("x1", "0%")
    .attr("y1", "0%")
    .attr("x2", "0%")
    .attr("y2", "100%");

  gradient.append("stop")
    .attr("offset", "0%")
    .attr("stop-color", props.color)
    .attr("stop-opacity", 0.3);

  gradient.append("stop")
    .attr("offset", "100%")
    .attr("stop-color", props.color)
    .attr("stop-opacity", 0);
    
  area = svg.append("path")
    .attr("fill", `url(#gradient-${chartId})`)

  updateChart()
}

const updateChart = () => {
  if (!svg || !props.data.length) return;

  const margin = { top: 10, right: 0, bottom: 20, left: 30 }
  const innerWidth = width - margin.left - margin.right
  const innerHeight = props.height - margin.top - margin.bottom

  const n = props.data.length
  
  x = d3.scaleLinear()
    .domain([0, n - 1])
    .range([0, innerWidth])

  // Y Scale (Dynamic domain with padding)
  const minVal = d3.min(props.data) || 0
  const maxVal = d3.max(props.data) || 1000 // Higher default for raw
  y = d3.scaleLinear()
    .domain([minVal - 50, maxVal + 50]) 
    .range([innerHeight, 0])

  line = d3.line()
    .x((d, i) => x(i))
    .y(d => y(d))
    .curve(d3.curveMonotoneX)

  const areaGenerator = d3.area()
    .x((d, i) => x(i))
    .y0(innerHeight)
    .y1(d => y(d))
    .curve(d3.curveMonotoneX)

  path.datum(props.data)
    .attr("d", line)
    
  area.datum(props.data)
    .attr("d", areaGenerator)
    
  svg.select(".y-axis")
    .call(d3.axisLeft(y).ticks(5))
    .call(g => g.select(".domain").remove())
    .call(g => g.selectAll(".tick line").attr("stroke-opacity", 0.1).attr("stroke", "#fff"))
    .call(g => g.selectAll(".tick text").attr("fill", "#8b949e"))
}
</script>

<template>
  <div class="chart-container">
    <svg ref="svgRef" width="100%"></svg>
  </div>
</template>

<style scoped>
.chart-container {
  width: 100%;
  overflow: hidden;
}
</style>
