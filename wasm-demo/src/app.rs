
use yew::{
    html, services::ConsoleService, Component, ComponentLink, Html,
    ShouldRender,
};

pub struct App {
    link: ComponentLink<Self>,
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            },
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            },
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            },
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=|_| Msg::Increment>
                        { "Increment" }
                    </button>
                    <button onclick=|_| Msg::Decrement>
                        { "Decrement" }
                    </button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment])>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>{ self.value }</p>
            </div>
        }
    }
}
