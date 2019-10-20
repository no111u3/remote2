use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {    
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ "Hello from Wasm Rust" }</p>
        }
    }
}
