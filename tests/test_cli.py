import contextlib
import io
import unittest
from unittest.mock import Mock, patch

from evdev import ecodes as e
from wheelpad_lab import main


class DeviceLifecycleTests(unittest.TestCase):
    def test_check_closes_devices_without_context_manager_support(self):
        for explicit in (False, True):
            for fail_creation in (False, True):
                with self.subTest(explicit=explicit, fail_creation=fail_creation):
                    device = Mock(spec=["name", "path", "capabilities", "close"])
                    device.name = "SynPS/2 Synaptics TouchPad"
                    device.path = "/dev/input/event-test"
                    device.capabilities.return_value = {
                        e.EV_ABS: [(e.ABS_X, None), (e.ABS_Y, None)],
                        e.EV_KEY: [e.BTN_TOUCH],
                    }
                    wheel = Mock(spec=["close"])
                    argv = ["wheelpad-lab", "--check"]
                    if explicit:
                        argv += ["--device", device.path]
                    with contextlib.ExitStack() as stack:
                        stack.enter_context(patch("sys.argv", argv))
                        stack.enter_context(patch("evdev.list_devices", return_value=[device.path]))
                        stack.enter_context(patch("evdev.InputDevice", return_value=device))
                        factory = stack.enter_context(patch("evdev.UInput", return_value=wheel))
                        if fail_creation:
                            factory.side_effect = OSError("uinput unavailable")
                        output = stack.enter_context(contextlib.redirect_stdout(io.StringIO()))
                        error = stack.enter_context(contextlib.redirect_stderr(io.StringIO()))
                        result = main()
                    device.close.assert_called_once_with()
                    if fail_creation:
                        self.assertEqual(result, 1)
                        self.assertIn("uinput unavailable", error.getvalue())
                        wheel.close.assert_not_called()
                    else:
                        self.assertIsNone(result)
                        self.assertIn("virtual wheel creation OK", output.getvalue())
                        wheel.close.assert_called_once_with()
