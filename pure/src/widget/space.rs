use crate::{Element, Tree, Widget};

use iced_native::event::{self, Event};
use iced_native::layout::{self, Layout};
use iced_native::mouse;
use iced_native::renderer;
use iced_native::text;
use iced_native::{Clipboard, Hasher, Length, Point, Rectangle, Shell};

pub use iced_native::widget::Space;

impl<'a, Message, Renderer> Widget<Message, Renderer> for Space
where
    Message: Clone,
    Renderer: text::Renderer,
{
    fn width(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::width(self)
    }

    fn height(&self) -> Length {
        <Self as iced_native::Widget<Message, Renderer>>::height(self)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        <Self as iced_native::Widget<Message, Renderer>>::layout(
            self, renderer, limits,
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        <Self as iced_native::Widget<Message, Renderer>>::on_event(
            self,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        <Self as iced_native::Widget<Message, Renderer>>::draw(
            self,
            renderer,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        <Self as iced_native::Widget<Message, Renderer>>::mouse_interaction(
            self,
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        <Self as iced_native::Widget<Message, Renderer>>::hash_layout(
            self, state,
        )
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Space
where
    Message: 'a + Clone,
    Renderer: text::Renderer + 'a,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}