<template>
  <div>
    <canvas id="canvas" width="480" height="320"></canvas>
  </div>
</template>

<script>
export default {
  data() {
    return {
      count: 0,
      spec: null,
      canvasCtx: null,
      width: 480,
      height: 320
    }
  },

  mounted() {
    this.initCanvas()
    this.draw()
    this.update()
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
        const sliceWidth = this.width * 1.0 / this.specArray.length
        let x = 0
        for(let i = 0; i < this.specArray.length; i++) {
          const y = this.height - this.specArray[i] * this.height
          if(i === 0) {
            this.canvasCtx.moveTo(x, y)
          } else {
            this.canvasCtx.lineTo(x, y)
          }
          x += sliceWidth
        }
        this.canvasCtx.lineTo(this.width, this.height)
        this.canvasCtx.stroke()
      }
    },

    getter: function() {
      const spec = external.invoke("getSpectrum")
      console.log(spec)
      this.spec = spec
    },

    update: function() {
      const self = this
      this.intervalid1 = setInterval(function(){
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
        return specArray.map((x) => (x - min) / (max - min)) // min-max normalization
      }
      return null
    }
  }
}
</script>
