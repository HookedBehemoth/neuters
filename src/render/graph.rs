use std::fmt::Write;

use crate::api::markit::Graph;

pub fn render_graph_svg(graph: &Graph) -> String {
    let mut s = String::new();
    for element in &graph.Elements {
        /* Determin highest and lowest point */
        let max = element.ComponentSeries[1].MaxValue;
        let min = element.ComponentSeries[2].MinValue;
        let diff = max - min;
        let values = &element.ComponentSeries[3].Values;
        let length = values.len();

        write!(s, r#"<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 {length} 100">"#).unwrap();

        s.push_str(r#"<polyline fill="none" stroke="orange" stroke-width="0.5" points=""#);
        /* Draw graph */
        for (i, value) in element.ComponentSeries[0].Values.iter().enumerate() {
            let value = 1.0 - (value - min) / diff;
            write!(s, "{},{} ", i, (value * 100.0) as u32).unwrap();
        }
        s.push_str(r#""/>"#);

        /* End SVG */
        s.push_str("</svg>");
    }
    s.to_owned()
}
