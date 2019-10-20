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
            <div id="root">
            <header class="row">
                <span class="logo">{ "Remote2" }</span>
            </header>
            <div>
                <span class="spinner"></span><span>{ "Developing in progress" }</span>
            </div>
            <footer>
                <p>{ "Draft version of WebUi for Remote2" }</p>
            </footer>
            </div>
        }
    }
}
