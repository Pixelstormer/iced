mod button;
mod column;
mod element;
mod text;
mod tree;

pub use button::Button;
pub use column::Column;
pub use element::Element;
pub use text::Text;
pub use tree::Tree;

use iced_native::event::{self, Event};
use iced_native::layout::{self, Layout};
use iced_native::mouse;
use iced_native::renderer;
use iced_native::{Clipboard, Hasher, Length, Point, Rectangle, Shell};

use std::any::{self, Any};

pub trait Widget<Message, Renderer> {
    fn tag(&self) -> any::TypeId;

    fn state(&self) -> Box<dyn Any>;

    fn children(&self) -> &[Element<Message, Renderer>];

    fn width(&self) -> Length;

    fn height(&self) -> Length;

    fn hash_layout(&self, state: &mut Hasher);

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node;

    fn draw(
        &self,
        state: &Tree<Message, Renderer>,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    );

    fn mouse_interaction(
        &self,
        _state: &Tree<Message, Renderer>,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::Idle
    }

    fn on_event(
        &mut self,
        _state: &mut Tree<Message, Renderer>,
        _event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        event::Status::Ignored
    }
}