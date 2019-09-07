#![feature(box_syntax)]

mod callback_handler;
mod engine;
mod events;
mod gfx;
mod platform;
mod init;
mod layer;
mod window;

pub mod input;

pub use callback_handler::CallbackHandler;
pub use engine::Engine;
pub use engine::EngineProps;
pub use events::Event;
pub use init::init;
pub use input::Button;
pub use input::InputState;
pub use layer::Layer;
pub use layer::LayerHandle;
pub use layer::LayerStack;
pub use gfx::Renderer;
pub use window::Window;
pub use window::WindowProps;

pub use gl;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
