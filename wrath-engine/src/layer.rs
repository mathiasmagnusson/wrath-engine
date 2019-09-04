use std::collections::VecDeque;
use std::ops::AddAssign;
use std::time::Duration;

use crate::Button;
use crate::Event;
use crate::Float;
use crate::Renderer;

pub struct LayerStack {
	inner: VecDeque<(Box<dyn Layer>, LayerHandle)>,
	handle_counter: LayerHandle,
}

impl LayerStack {
	pub fn new() -> Self {
		Self {
			inner: VecDeque::default(),
			handle_counter: LayerHandle(1),
		}
	}
	pub fn submit(&mut self, mut event: Box<dyn Event>) {
		for layer in self.inner.iter_mut() {
			event.dispatch(layer.0.as_mut());
			if event.is_handled() {
				break;
			}
		}
	}
	pub fn call_update(&mut self, dt: Duration) {
		for layer in self.inner.iter_mut() {
			layer.0.on_update(dt);
		}
	}
	pub fn call_render(&mut self, renderer: &mut Renderer) {
		for layer in self.inner.iter_mut() {
			layer.0.on_render(renderer);
		}
	}
	pub fn push_back(&mut self, layer: Box<dyn Layer>) -> LayerHandle {
		let handle = self.handle_counter;
		self.handle_counter += 1;
		self.inner.push_back((layer, handle));
		handle
	}
	pub fn push_front(&mut self, layer: Box<dyn Layer>) -> LayerHandle {
		let handle = self.handle_counter;
		self.handle_counter += 1;
		self.inner.push_front((layer, handle));
		handle
	}
	pub fn remove_layer(&mut self, handle: LayerHandle) -> bool {
		for i in 0..self.inner.len() {
			if self.inner[i].1 == handle {
				self.inner.remove(i);
				return true;
			}
		}
		false
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LayerHandle(u32);

impl LayerHandle {
	pub fn none() -> Self {
		LayerHandle(0)
	}
}

impl AddAssign<u32> for LayerHandle {
	fn add_assign(&mut self, rhs: u32) {
		self.0 += rhs;
	}
}

#[allow(unused_variables)]
pub trait Layer {
	fn on_attach(&mut self) {}
	fn on_detach(&mut self) {}
	fn on_update(&mut self, dt: Duration) {}
	fn on_render(&mut self, renderer: &mut Renderer) {}
	fn on_window_close_requested(&mut self) {}
	fn on_window_resize(&mut self, size: (u32, u32)) {}
	fn on_key_press(&mut self, button: Button, repeat: bool) -> bool { false }
	fn on_key_release(&mut self, button: Button) -> bool { false }
	fn on_text_written(&mut self, which: char) -> bool { false }
	fn on_mouse_down(&mut self, button: Button) -> bool { false }
	fn on_mouse_up(&mut self, button: Button) -> bool { false }
	fn on_mouse_move(&mut self, position: (u32, u32), delta: (i32, i32)) -> bool { false }
	fn on_mouse_scroll(&mut self, delta: (Float, Float)) -> bool { false }
}
