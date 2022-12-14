use yew::prelude::*;
fn main() {
    yew::start_app::<Model>();
}

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 2 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!(
            <div>
                <button onclick = {link.callback(|_| Msg::AddOne)}>
                {"+1"}
                </button>
                <p>
                {self.value}
                </p>
            </div>
        )
    }
}
