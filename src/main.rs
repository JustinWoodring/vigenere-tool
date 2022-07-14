use cursive::{
    traits::{Nameable, Resizable},
    views::{Button, EditView, LinearLayout, PaddedView, Panel, RadioGroup, TextView, ViewRef},
};

mod better_text_area;

#[derive(PartialEq, Eq)]
pub enum Mode {
    Encode,
    Decode,
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();
    siv.set_user_data("".to_string());

    let mut encode = better_text_area::TextArea::new();
    encode.set_on_edit(|cursive, value| {
        let key = cursive.user_data::<String>().unwrap().clone();
        cursive.call_on_name("decode", |view: &mut better_text_area::TextArea| {
            view.set_content(vigenere_encode(value, key.as_str()));
        });
    });

    let encoder = LinearLayout::vertical()
        .child(TextView::new("Text to Encode:").with_name("encode_label"))
        .child(encode.with_name("encode").fixed_size((30, 5)));

    let mut decode = better_text_area::TextArea::new();
    decode.set_on_edit(|cursive, value| {
        let key = cursive.user_data::<String>().unwrap().clone();
        cursive.call_on_name("encode", |view: &mut better_text_area::TextArea| {
            view.set_content(vigenere_decode(value, key.as_str()));
        });
    });

    let decoder = LinearLayout::vertical()
        .child(TextView::new("Encoded Output:").with_name("decode_label"))
        .child(decode.disabled().with_name("decode").fixed_size((30, 5)));

    let mut key_input = EditView::new();
    key_input.set_on_edit(|cursive, key, _size| {
        let t = key
            .replace(|c: char| !c.is_ascii_alphabetic(), "")
            .to_lowercase();
        cursive.set_user_data(t.clone());

        let mut encode: ViewRef<better_text_area::TextArea> = cursive.find_name("encode").unwrap();
        let mut decode: ViewRef<better_text_area::TextArea> = cursive.find_name("decode").unwrap();
        if encode.is_enabled() {
            decode.set_content(vigenere_encode(encode.get_content(), t.as_str()));
        } else {
            encode.set_content(vigenere_decode(decode.get_content(), t.as_str()));
        }
    });

    let key = LinearLayout::vertical()
        .child(TextView::new("Key:"))
        .child(key_input.with_name("key").fixed_width(30));

    let padded_encoder = PaddedView::lrtb(0, 0, 1, 1, encoder);
    let padded_decoder = PaddedView::lrtb(0, 0, 0, 1, decoder);
    let padded_key = PaddedView::lrtb(0, 0, 0, 1, key);

    let mut toggle = RadioGroup::new();
    toggle.set_on_change(|cursive, value| {
        if value == &Mode::Encode {
            cursive.call_on_name("decode", |view: &mut better_text_area::TextArea| {
                view.disable()
            });
            cursive.call_on_name("encode", |view: &mut better_text_area::TextArea| {
                view.enable()
            });

            cursive.call_on_name("encode_label", |view: &mut TextView| {
                view.set_content("Text to Encode:")
            });
            cursive.call_on_name("decode_label", |view: &mut TextView| {
                view.set_content("Encoded Output:")
            });
        } else {
            cursive.call_on_name("decode", |view: &mut better_text_area::TextArea| {
                view.enable()
            });
            cursive.call_on_name("encode", |view: &mut better_text_area::TextArea| {
                view.disable()
            });

            cursive.call_on_name("encode_label", |view: &mut TextView| {
                view.set_content("Decoded Output:")
            });
            cursive.call_on_name("decode_label", |view: &mut TextView| {
                view.set_content("Text to Decode:")
            });
        }
    });

    let encode_toggle = toggle.button(Mode::Encode, "Encode");
    let decode_toggle = toggle.button(Mode::Decode, "Decode");

    let layout = LinearLayout::vertical()
        .child(padded_encoder)
        .child(padded_decoder)
        .child(padded_key)
        .child(
            LinearLayout::horizontal()
                .child(PaddedView::lrtb(0, 2, 0, 0, encode_toggle))
                .child(decode_toggle),
        );

    let mut panel = Panel::new(PaddedView::lrtb(2, 2, 0, 0, layout));
    panel.set_title("Vigenere Cipher Encoder/Decoder");

    let screen1 = siv.add_active_screen();
    siv.add_layer(panel);

    let _screen2 = siv.add_active_screen();
    siv.add_layer(Panel::new(PaddedView::lrtb(
        2,
        2,
        2,
        2,
        LinearLayout::vertical()
            .child(
                TextView::new(
                    r#"
Vigenere Cipher Encoding / Decoding Tool
Created by Justin Woodring
Licensed under MIT

Press Ctrl-C to quit at any time.
"#,
                )
                .center()
                .fixed_height(10),
            )
            .child(Button::new("Ok", move |s| s.set_screen(screen1.clone()))),
    )));

    // Starts the event loop.
    siv.run();
}

fn vigenere_encode(input: &str, key: &str) -> String {
    let mut output = String::new();

    let mut key_iter = key.chars();
    for my_char in input.chars() {
        let mut output_char = my_char;
        output_char = output_char.to_ascii_lowercase();
        let mut next_char = key_iter.next();
        if next_char.is_none() {
            key_iter = key.chars();
            next_char = key_iter.next();
        }

        match next_char {
            Some(key_char) => {
                if key_char.is_ascii_alphabetic() && my_char.is_ascii_alphabetic() {
                    let lower_case_key_char = key_char.to_ascii_lowercase();
                    let shift = (lower_case_key_char as u8) - ('a' as u8);
                    if (output_char as u8 + shift) > ('z' as u8) {
                        output_char =
                            (('a' as u8) + ((output_char as u8 + shift) - 'z' as u8)) as char;
                    } else {
                        output_char = (output_char as u8 + shift) as char;
                    }
                }
            }
            None => {}
        }

        if my_char.is_uppercase() {
            output_char = output_char.to_ascii_uppercase();
        }
        output.push(output_char);
    }

    output
}

fn vigenere_decode(input: &str, key: &str) -> String {
    let mut output = String::new();

    let mut key_iter = key.chars();
    for my_char in input.chars() {
        let mut output_char = my_char;
        output_char = output_char.to_ascii_lowercase();
        let mut next_char = key_iter.next();
        if next_char.is_none() {
            key_iter = key.chars();
            next_char = key_iter.next();
        }

        match next_char {
            Some(key_char) => {
                if key_char.is_ascii_alphabetic() && my_char.is_ascii_alphabetic() {
                    let lower_case_key_char = key_char.to_ascii_lowercase();
                    let shift = (lower_case_key_char as u8) - ('a' as u8);
                    if (output_char as u8 as i32 - shift as i32) < ('a' as u8 as i32) {
                        output_char = (('z' as u8 as i32)
                            - (('a' as u8 as i32) - (output_char as u8 as i32 - shift as i32)))
                            as u8 as char;
                    } else {
                        output_char = (output_char as u8 - shift) as char;
                    }
                }
            }
            None => {}
        }

        if my_char.is_uppercase() {
            output_char = output_char.to_ascii_uppercase();
        }
        output.push(output_char);
    }

    output
}
