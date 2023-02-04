# wasm-echarts-ex

echarts在rust语言中的初步使用例子。

## 实现方法

可分为两类，1）使用胶水/代理脚本`echats-proxy.js`，2）只使用`rust`代码

### 实现1：`echarts_init`函数

代理脚本`echats-proxy.js`负责组装并显示曲线，`rust`提供数据。

参数：
- id -- 字符串
- opt -- x/y轴的数据。{}



### 实现2：`echarts_init2`函数

代理脚本`echats-proxy.js`负责组装并显示曲线，`rust`提供完整图标配置数据。

参数：
- el -- Element 网页元素
- opt -- 整个曲线配置


### 实现3：`echarts_init3`函数

