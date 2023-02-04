use gloo_utils::format::JsValueSerdeExt;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use web_sys::Element;

// https://echarts.apache.org/en/api.html#echarts

#[wasm_bindgen(module = "/echarts-proxy.js")]
extern "C" {
    fn echarts_init(id: &str, options: &JsValue);
    fn echarts_init2(el: &Element, options: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    pub type ECharts;

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn echart_init3(dom: &Element, theme: &JsValue, opts: &JsValue) -> ECharts;

    #[wasm_bindgen(structural, method)]
    pub fn setOption(this: &ECharts, option: &JsValue);
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(start)]
fn run() {
    // -- echarts init
    {
        let data = r#"{
            "month": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
            "value": [250, 230, 124, 218, 135, 147, 260]
        }"#;
        let v: Value = serde_json::from_str(data).unwrap();
        let opt = JsValue::from_serde(&v).unwrap_or_default();
        echarts_init("plot-div", &opt);
    }

    // -- echarts init2
    {
        let el = gloo_utils::document().create_element("div").unwrap();
        el.set_id("plot-div2");
        
        let data = r#"{"xAxis":{"type":"category","data":["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]},"yAxis":{"type":"value"},"series":[{"data":[820,932,901,934,1290,1330,1320],"type":"line","smooth":true}]}"#;
        let v: Value = serde_json::from_str(data).unwrap();
        let opt = JsValue::from_serde(&v).unwrap_or_default();
        echarts_init2(&el, &opt);
        
        let dyna_plot = gloo_utils::document().get_element_by_id("dyna_plot").unwrap();
        dyna_plot.append_child(&el).ok();
    }

    // -- echarts init3
    {
        let el = gloo_utils::document().create_element("div").unwrap();
        el.set_id("plot-div3");
        
        let init_str = r#"{"width":"400px","height":"300px"}"#;
        let v0: Value = serde_json::from_str(init_str).unwrap();
        let init_options = JsValue::from_serde(&v0).unwrap_or_default();
        let inst = echart_init3(&el, &JsValue::null(), &init_options);

        let data = r#"
            {"xAxis":{"type":"category","data":["Mon","Tue","Wed","Thu","Fri","Sat","Sun"]},"yAxis":{"type":"value"},"series":[{"data":[120,200,150,80,70,110,130],"type":"bar","showBackground":true,"backgroundStyle":{"color":"rgba(180, 180, 180, 0.2)"}}]}
        "#;
        let v: Value = serde_json::from_str(data).unwrap();
        let option = JsValue::from_serde(&v).unwrap_or_default();
        inst.setOption(&option);
        
        let dyna_plot2 = gloo_utils::document().get_element_by_id("dyna_plot2").unwrap();
        dyna_plot2.append_child(&el).ok();
        log("plot-div3");
    }

}
