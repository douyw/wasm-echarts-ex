// solution 1) use glue function echarts_init, partial data provided in rust code.
// see `echarts_init` fn import and usage in rust code.
// id: string; opt: partial data, json data format: {month, value} 
export function echarts_init(id, opt) {
  console.log("echarts_init -- ", id, "opt: ", opt)
  
  opt = opt || 
    { month: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
      value: [150, 230, 224, 218, 135, 147, 260]
    }

  let el = document.getElementById(id);
  let options = { width: "400px", height: "300px" }
  let inst = echarts.init(el, null, options);
  let option = {
    xAxis: {
      type: 'category',
      data: opt.month
    },
    yAxis: {
      type: 'value'
    },
    series: [
      {
        data: opt.value,
        type: 'line'
      }
    ]
  };
  inst.setOption(option)
}

// solution 2) use glue function echarts_init2, the entire chart option provided in rust code
// see `echarts_init2` fn import and usage in rust code
// el: Element; opt: chart option, json data format: {xAxis:{type, data}, yAxis:{type},series:[{data,type}]}
export function echarts_init2(el, opt) {
  let inst = echarts.init(el, null, {width: "400px", height: "300px"})
  inst.setOption(opt)
}

// solution 3) use rust code only, see echarts_init3.
