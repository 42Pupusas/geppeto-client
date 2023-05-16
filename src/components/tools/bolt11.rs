use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};
use web_sys::{window, Element};
use yew::prelude::*;

pub fn create_qr_code(data: &str) -> String {
    let code = QrCode::with_version(data.as_bytes(), Version::Normal(40), EcLevel::H).unwrap();
    let image = code
        .render::<svg::Color>()
        .max_dimensions(280, 280)
        .build();

    println!("{}", image);
    image
}

#[derive(Properties, Clone, PartialEq)]
pub struct MyComponentProps {
    pub data: String,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyComponentProps) -> Html {
    let node_ref = NodeRef::default();
    let data = props.data.clone();

    let node_ref_clone = node_ref.clone();

    use_effect(move || {
        let svg_string = create_qr_code(&data);
        let node_ref = node_ref.clone();
        let svg_string = svg_string.clone();
        if let Some(div) = node_ref.cast::<Element>() {
            let document = window().unwrap().document().unwrap();
            let svg_element: Element = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
                .unwrap();
            svg_element.set_inner_html(&svg_string);
            div.append_child(&svg_element).unwrap();
        }
        || {}
    });

    html! {
        <div class="svg-container" ref={node_ref_clone}>
        </div>
    }
}
