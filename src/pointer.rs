//! Route whole contacts: rim contacts scroll; central contacts stay native.
use evdev::{AbsoluteAxisCode as Abs, EventType, InputEvent, KeyCode as Key};

#[derive(Default)]
pub struct PointerFilter {
    scrolling: Option<bool>,
    buttons: u8,
}
impl PointerFilter {
    pub fn frame(
        &mut self,
        events: &[InputEvent],
        point: Option<(i32, i32)>,
        touching: bool,
        multi: u8,
    ) -> (Vec<InputEvent>, bool) {
        for event in events {
            if event.event_type() == EventType::KEY {
                for (bit, key) in [Key::BTN_LEFT, Key::BTN_RIGHT, Key::BTN_MIDDLE]
                    .iter()
                    .enumerate()
                {
                    if event.code() == key.0 {
                        if event.value() != 0 {
                            self.buttons |= 1 << bit;
                        } else {
                            self.buttons &= !(1 << bit);
                        }
                    }
                }
            }
        }
        if touching && self.scrolling.is_none() {
            // Keep this decision until release, avoiding jumps and accidental rim taps.
            self.scrolling =
                Some(multi == 0 && self.buttons == 0 && point.is_some_and(crate::gesture::on_rim));
        }
        let scrolling = self.scrolling.unwrap_or(false);
        let output = events
            .iter()
            .filter(|event| event.event_type() != EventType::SYNCHRONIZATION)
            .map(|event| {
                let (kind, code) = (event.event_type(), event.code());
                let value = if scrolling
                    && kind == EventType::KEY
                    && [
                        Key::BTN_TOUCH,
                        Key::BTN_TOOL_FINGER,
                        Key::BTN_TOOL_DOUBLETAP,
                        Key::BTN_TOOL_TRIPLETAP,
                        Key::BTN_TOOL_QUADTAP,
                        Key::BTN_TOOL_QUINTTAP,
                    ]
                    .iter()
                    .any(|k| k.0 == code)
                {
                    0
                } else if scrolling
                    && kind == EventType::ABSOLUTE
                    && code == Abs::ABS_MT_TRACKING_ID.0
                {
                    -1
                } else if scrolling
                    && kind == EventType::ABSOLUTE
                    && [Abs::ABS_PRESSURE, Abs::ABS_TOOL_WIDTH, Abs::ABS_MT_PRESSURE]
                        .iter()
                        .any(|a| a.0 == code)
                {
                    0
                } else {
                    event.value()
                };
                InputEvent::new(kind.0, code, value)
            })
            .collect();
        if !touching {
            self.scrolling = None;
        }
        (output, scrolling)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event(kind: EventType, code: u16, value: i32) -> InputEvent {
        InputEvent::new(kind.0, code, value)
    }
    #[test]
    fn rim_contact_hides_touch_and_mt_but_keeps_buttons_and_slots() {
        let mut f = PointerFilter::default();
        let events = vec![
            event(EventType::KEY, Key::BTN_TOUCH.0, 1),
            event(EventType::ABSOLUTE, Abs::ABS_MT_SLOT.0, 1),
            event(EventType::ABSOLUTE, Abs::ABS_MT_TRACKING_ID.0, 42),
            event(EventType::ABSOLUTE, Abs::ABS_PRESSURE.0, 90),
        ];
        let (out, scroll) = f.frame(&events, Some((5300, 2942)), true, 0);
        assert!(scroll);
        assert_eq!(
            out.iter().map(|e| e.value()).collect::<Vec<_>>(),
            [0, 1, -1, 0]
        );
        let click = [event(EventType::KEY, Key::BTN_LEFT.0, 1)];
        assert_eq!(f.frame(&click, Some((3600, 2942)), true, 0).0[0].value(), 1);
        assert!(f.frame(&[], None, false, 0).1);
        assert!(!f.frame(&events, Some((3600, 2942)), true, 0).1);
    }
    #[test]
    fn central_and_multifinger_contacts_remain_pointer_until_release() {
        for (point, multi) in [(Some((3600, 2942)), 0), (Some((5300, 2942)), 1)] {
            let mut f = PointerFilter::default();
            let events = [event(EventType::KEY, Key::BTN_TOUCH.0, 1)];
            assert_eq!(f.frame(&events, point, true, multi).0[0].value(), 1);
            assert!(!f.frame(&[], Some((5300, 2942)), true, 0).1);
        }
    }
}
