<template>
  <div>
    <p>{{count}}</p>
    <button type="button" name="button" v-on:click="countUp()">Count UP</button>
  </div>
</template>

<script>
export default {
  data() {
    return {
      count: 0
    }
  },

  methods: {
    countUp: function() {
      this.count += 1
      external.invoke(JSON.stringify({ 'cmd': 'countUp', 'count': this.count }))
    }
  }
}
</script>
