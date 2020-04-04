use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
  events: sdl2::EventPump,
  pub keys: [bool; 16]
}

impl InputDriver {
  pub fn new(sdl_context: &sdl2::Sdl) -> Self {
    InputDriver {
      events: sdl_context.event_pump().unwrap(),
      keys: [false; 16]
    }
  }

  pub fn poll(&mut self) -> Result<[bool; 16], ()> {
    for event in self.events.poll_iter() {
      if let Event::Quit { .. } = event {
        return Err(());
      };
    }

    let keys: Vec<Keycode> = self.events
      .keyboard_state()
      .pressed_scancodes()
      .filter_map(Keycode::from_scancode)
      .collect();

    for key in keys {
      let index = match key {
        Keycode::Num1 => Some(0x01),
        Keycode::Num2 => Some(0x02),
        Keycode::Num3 => Some(0x03),
        Keycode::Num4 => Some(0x0c),
        Keycode::Q => Some(0x04),
        Keycode::W => Some(0x05),
        Keycode::E => Some(0x06),
        Keycode::R => Some(0x0d),
        Keycode::A => Some(0x07),
        Keycode::S => Some(0x08),
        Keycode::D => Some(0x09),
        Keycode::F => Some(0x0e),
        Keycode::Z => Some(0x0a),
        Keycode::X => Some(0x00),
        Keycode::C => Some(0x0b),
        Keycode::V => Some(0x0f),
        _ => None,
      };

      if let Some(i) = index {
        self.keys[i] = true;
      }
    }

    Ok(self.keys)
  }
}