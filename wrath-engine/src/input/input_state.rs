pub static mut INPUT_STATE: InputState = InputState {
	mouse_position: (0, 0),
	mouse_left: false,
	mouse_right: false,
	mouse_middle: false,
	mouse4: false,
	mouse5: false,
	mouse6: false,
	mouse7: false,
	mouse8: false,
	l_shift: false,
	l_ctrl: false,
	l_alt: false,
	l_super: false,
	r_shift: false,
	r_ctrl: false,
	r_alt: false,
	r_super: false,
	tab: false,
	caps_lock: false,
	backspace: false,
	esc: false,
	f1: false,
	f2: false,
	f3: false,
	f4: false,
	f5: false,
	f6: false,
	f7: false,
	f8: false,
	f9: false,
	f10: false,
	f11: false,
	f12: false,
	print_screen: false,
	scroll_lock: false,
	pause: false,
	insert: false,
	delete: false,
	home: false,
	end: false,
	pg_up: false,
	pg_down: false,
	menu: false,
	arrow_left: false,
	arrow_up: false,
	arrow_right: false,
	arrow_down: false,
	tilde: false,
	num1: false,
	num2: false,
	num3: false,
	num4: false,
	num5: false,
	num6: false,
	num7: false,
	num8: false,
	num9: false,
	num0: false,
	q: false,
	w: false,
	e: false,
	r: false,
	t: false,
	y: false,
	u: false,
	i: false,
	o: false,
	p: false,
	a: false,
	s: false,
	d: false,
	f: false,
	g: false,
	h: false,
	j: false,
	k: false,
	l: false,
	z: false,
	x: false,
	c: false,
	v: false,
	b: false,
	n: false,
	m: false,
	space: false,
	minus: false,
	equals: false,
	bracket_left: false,
	bracket_right: false,
	backslash: false,
	semicolon: false,
	apostrophe: false,
	enter: false,
	comma: false,
	period: false,
	slash: false,
	numpad0: false,
	numpad1: false,
	numpad2: false,
	numpad3: false,
	numpad4: false,
	numpad5: false,
	numpad6: false,
	numpad7: false,
	numpad8: false,
	numpad9: false,
	numlock: false,
	numpad_dec: false,
	numpad_div: false,
	numpad_mult: false,
	numpad_sub: false,
	numpad_add: false,
	numpad_eq: false,
	numpad_enter: false,
};

pub struct InputState {
	pub mouse_position: (u32, u32),
	pub mouse_left: bool,
	pub mouse_right: bool,
	pub mouse_middle: bool,
	pub mouse4: bool,
	pub mouse5: bool,
	pub mouse6: bool,
	pub mouse7: bool,
	pub mouse8: bool,
	pub l_shift: bool,
	pub l_ctrl: bool,
	pub l_alt: bool,
	pub l_super: bool,
	pub r_shift: bool,
	pub r_ctrl: bool,
	pub r_alt: bool,
	pub r_super: bool,
	pub tab: bool,
	pub caps_lock: bool,
	pub backspace: bool,
	pub esc: bool,
	pub f1: bool,
	pub f2: bool,
	pub f3: bool,
	pub f4: bool,
	pub f5: bool,
	pub f6: bool,
	pub f7: bool,
	pub f8: bool,
	pub f9: bool,
	pub f10: bool,
	pub f11: bool,
	pub f12: bool,
	pub print_screen: bool,
	pub scroll_lock: bool,
	pub pause: bool,
	pub insert: bool,
	pub delete: bool,
	pub home: bool,
	pub end: bool,
	pub pg_up: bool,
	pub pg_down: bool,
	pub menu: bool,
	pub arrow_left: bool,
	pub arrow_up: bool,
	pub arrow_right: bool,
	pub arrow_down: bool,
	pub tilde: bool,
	pub num1: bool,
	pub num2: bool,
	pub num3: bool,
	pub num4: bool,
	pub num5: bool,
	pub num6: bool,
	pub num7: bool,
	pub num8: bool,
	pub num9: bool,
	pub num0: bool,
	pub q: bool,
	pub w: bool,
	pub e: bool,
	pub r: bool,
	pub t: bool,
	pub y: bool,
	pub u: bool,
	pub i: bool,
	pub o: bool,
	pub p: bool,
	pub a: bool,
	pub s: bool,
	pub d: bool,
	pub f: bool,
	pub g: bool,
	pub h: bool,
	pub j: bool,
	pub k: bool,
	pub l: bool,
	pub z: bool,
	pub x: bool,
	pub c: bool,
	pub v: bool,
	pub b: bool,
	pub n: bool,
	pub m: bool,
	pub space: bool,
	pub minus: bool,
	pub equals: bool,
	pub bracket_left: bool,
	pub bracket_right: bool,
	pub backslash: bool,
	pub semicolon: bool,
	pub apostrophe: bool,
	pub enter: bool,
	pub comma: bool,
	pub period: bool,
	pub slash: bool,
	pub numpad0: bool,
	pub numpad1: bool,
	pub numpad2: bool,
	pub numpad3: bool,
	pub numpad4: bool,
	pub numpad5: bool,
	pub numpad6: bool,
	pub numpad7: bool,
	pub numpad8: bool,
	pub numpad9: bool,
	pub numlock: bool,
	pub numpad_dec: bool,
	pub numpad_div: bool,
	pub numpad_mult: bool,
	pub numpad_sub: bool,
	pub numpad_add: bool,
	pub numpad_eq: bool,
	pub numpad_enter: bool,
}

pub fn get_mouse_position() -> (u32, u32) {
	unsafe {
		INPUT_STATE.mouse_position
	}
}
