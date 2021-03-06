use crate::live::{Requirement, ResponseEvt};
use crate::widgets::{Reqs, View, Widget, WidgetModel};
use protocol::{Id, Reaction, Value};
use yew::{html, Properties, ShouldRender};

pub type DynamicWidget = WidgetModel<Model>;

pub struct Model {
    value: Value,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: Id,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(_: &Self::Properties) -> Self {
        Self {
            value: Value::Nothing,
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        let id = props.id.clone();
        Some(vec![Requirement::AssignUpdate(id)].into_iter().collect())
    }

    fn handle_incoming(&mut self, event: ResponseEvt) -> ShouldRender {
        if let ResponseEvt::Reaction(Reaction::Delta(delta)) = event {
            log::info!("Changing value: {:?}", delta);
            self.value = delta.value;
            true
        } else {
            false
        }
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <p class="dynamic",>{ &self.value }</p>
        }
    }
}
