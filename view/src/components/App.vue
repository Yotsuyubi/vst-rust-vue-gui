<template>
  <div>
    <canvas id="canvas" :width="width" :height="height"></canvas>
  </div>
</template>

<script>
export default {
  data() {
    return {
      spec: null,
      canvasCtx: null,
      width: 460,
      height: 300
    }
  },

  mounted() {
    this.initCanvas()
    this.draw()
    this.update()
  },

  beforeDestroy() {
    console.log('beforeDestroy');
    clearInterval(this.intervalid1)
  },

  methods: {
    initCanvas: function() {
      const canvas = document.getElementById("canvas")
      this.canvasCtx = canvas.getContext("2d")
      this.canvasCtx.clearRect(0, 0, this.width, this.height)
    },

    draw: function() {
      if (this.specArray != null) {
        this.canvasCtx.fillStyle = 'rgb(200, 200, 200)'
        this.canvasCtx.clearRect(0, 0, this.width, this.height)
        this.canvasCtx.lineWidth = 2
        this.canvasCtx.strokeStyle = 'rgb(0, 0, 0)'
        this.canvasCtx.beginPath()
        const initWidth = this.width / Math.log(this.specArray.length / 2)
        let x = 0
        for(let i = 0; i < this.specArray.length / 2; i++) {
          const y = this.height - this.specArray[i] * this.height
          if(i === 0) {
            this.canvasCtx.moveTo(x, y)
          } else {
            this.canvasCtx.lineTo(x, y)
          }
          x += initWidth / (i+1)
        }
        this.canvasCtx.lineTo(this.width, this.height)
        this.canvasCtx.stroke()
      }
    },

    getter: function() {
      const spec = external.invoke("getSpectrum")
      this.spec = spec
    },

    update: function() {
      const self = this
      this.intervalid1 = setInterval(function() {
        self.getter()
        self.draw()
      }, 1000/30)
    }
  },

  computed: {
    specArray: function() {
      if (this.spec != null) {
        const array = this.spec.split(',')
        const specArray = array.map((x) => Number(x))
        const min = specArray.reduce((a, b) => Math.min(a, b))
        const max = specArray.reduce((a, b) => Math.max(a, b))
        // return specArray.map((x) => (x - min) / (max - min)) // min-max normalization
        return specArray.map((x) => x/(specArray.length/2))
      }
      return null
    }
  }
}
</script>
