// Copyright (C) 2020 Mathieu Duponchelle <mathieu@centricular.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Library General Public
// License as published by the Free Software Foundation; either
// version 2 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Library General Public License for more details.
//
// You should have received a copy of the GNU Library General Public
// License along with this library; if not, write to the
// Free Software Foundation, Inc., 51 Franklin Street, Suite 500,
// Boston, MA 02110-1335, USA.

use gst::prelude::*;

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        gst::init().unwrap();
        gstrsregex::plugin_register_static().expect("regex test");
    });
}

#[test]
fn test_replace_all() {
    init();

    let input = b"crap that mothertrapper";

    let expected_output = "trap that mothertrapper";

    let mut h = gst_check::Harness::new("regex");

    {
        let regex = h.element().expect("Could not create regex");

        let command = gst::Structure::builder("replace-all")
            .field("pattern", "crap")
            .field("replacement", "trap")
            .build();

        let commands = gst::Array::from(vec![command.to_send_value()]);

        regex.set_property("commands", &commands);
    }

    h.set_src_caps_str("text/x-raw, format=utf8");

    let buf = {
        let mut buf = gst::Buffer::from_mut_slice(Vec::from(&input[..]));
        let buf_ref = buf.get_mut().unwrap();
        buf_ref.set_pts(gst::ClockTime::from_seconds(0));
        buf_ref.set_duration(gst::ClockTime::from_seconds(2));
        buf
    };

    assert_eq!(h.push(buf), Ok(gst::FlowSuccess::Ok));

    let buf = h.pull().expect("Couldn't pull buffer");

    assert_eq!(buf.pts(), Some(gst::ClockTime::ZERO));
    assert_eq!(buf.duration(), Some(2 * gst::ClockTime::SECOND));

    let map = buf.map_readable().expect("Couldn't map buffer readable");

    assert_eq!(
        std::str::from_utf8(map.as_ref()),
        std::str::from_utf8(expected_output.as_ref())
    );
}
