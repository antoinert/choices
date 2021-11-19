use yew::prelude::*;
use yew_styles::button::Button;
use yew_styles::styles::{Palette, Style};
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{Item, AlignSelf, ItemLayout};
enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
          <Container direction=Direction::Row wrap=Wrap::Wrap class_name="align-item">
            <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexStart>
            <Button
            onclick_signal=self.link.callback(|_| Msg::AddOne)
            class_name="hello-world"
            button_palette=Palette::Standard
            button_style=Style::Light
        >{"Click me!"}</Button>
            </Item>
            <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::Center>
            <Button
            onclick_signal=self.link.callback(|_| Msg::AddOne)
            class_name="hello-world"
            button_palette=Palette::Standard
            button_style=Style::Light
        >{"Hello!"}</Button>
            </Item>
            <Item layouts=vec!(ItemLayout::ItXs(4)) align_self=AlignSelf::FlexEnd>
            <Button
            onclick_signal=self.link.callback(|_| Msg::AddOne)
            class_name="hello-world"
            button_palette=Palette::Standard
            button_style=Style::Light
        >{"Foobar"}</Button>
            </Item>
        </Container>
        }
    }
}

#[allow(dead_code)]
fn main() {
    yew::start_app::<Model>();
}