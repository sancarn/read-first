#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

const ONE_MEGABYTE: usize = 1024 * 1024;
const WINDOW_MARGIN: u32 = 8;
const LABEL_HEIGHT: u32 = 18;
const CHECKBOX_HEIGHT: u32 = 22;
const LABEL_GAP: u32 = 2;
const TEXTBOX_GAP: u32 = 6;

#[derive(Copy, Clone)]
enum ReadMode {
    FirstLine,
    FirstMegabyte,
}

struct ReadResult {
    details: String,
    text: String,
}

#[allow(dead_code)]
struct ViewerUi {
    window: nwg::Window,
    app_icon: nwg::Icon,
    file_label: nwg::Label,
    details_label: nwg::Label,
    wrap_checkbox: nwg::CheckBox,
    nowrap_text_box: nwg::RichTextBox,
    wrap_text_box: nwg::RichTextBox,
}

impl ViewerUi {
    fn build(mode: ReadMode, file_path: &str, result: ReadResult) -> Result<Self, nwg::NwgError> {
        let title = match mode {
            ReadMode::FirstLine => "first-line",
            ReadMode::FirstMegabyte => "first-megabyte",
        };

        let mut app_icon = nwg::Icon::default();
        nwg::Icon::builder()
            .source_bin(Some(include_bytes!("../assets/app.ico")))
            .build(&mut app_icon)?;

        let mut window = nwg::Window::default();
        let mut file_label = nwg::Label::default();
        let mut details_label = nwg::Label::default();
        let mut wrap_checkbox = nwg::CheckBox::default();
        let mut nowrap_text_box = nwg::RichTextBox::default();
        let mut wrap_text_box = nwg::RichTextBox::default();

        nwg::Window::builder()
            .size((900, 650))
            .position((300, 100))
            .title(title)
            .icon(Some(&app_icon))
            .build(&mut window)?;

        nwg::Label::builder()
            .text(file_path)
            .parent(&window)
            .build(&mut file_label)?;

        nwg::Label::builder()
            .text(&result.details)
            .parent(&window)
            .build(&mut details_label)?;

        nwg::CheckBox::builder()
            .text("Wrap lines")
            .check_state(nwg::CheckBoxState::Unchecked)
            .parent(&window)
            .build(&mut wrap_checkbox)?;

        nwg::RichTextBox::builder()
            .text(&result.text)
            .flags(
                nwg::RichTextBoxFlags::VISIBLE
                    | nwg::RichTextBoxFlags::TAB_STOP
                    | nwg::RichTextBoxFlags::VSCROLL
                    | nwg::RichTextBoxFlags::HSCROLL
                    | nwg::RichTextBoxFlags::AUTOVSCROLL
                    | nwg::RichTextBoxFlags::AUTOHSCROLL
                    | nwg::RichTextBoxFlags::SAVE_SELECTION,
            )
            .readonly(true)
            .focus(true)
            .parent(&window)
            .build(&mut nowrap_text_box)?;

        nwg::RichTextBox::builder()
            .text(&result.text)
            .flags(
                nwg::RichTextBoxFlags::TAB_STOP
                    | nwg::RichTextBoxFlags::VSCROLL
                    | nwg::RichTextBoxFlags::AUTOVSCROLL
                    | nwg::RichTextBoxFlags::SAVE_SELECTION,
            )
            .readonly(true)
            .parent(&window)
            .build(&mut wrap_text_box)?;

        file_label.set_position(WINDOW_MARGIN as i32, WINDOW_MARGIN as i32);
        details_label.set_position(
            WINDOW_MARGIN as i32,
            (WINDOW_MARGIN + LABEL_HEIGHT + LABEL_GAP) as i32,
        );

        let ui = Self {
            window,
            app_icon,
            file_label,
            details_label,
            wrap_checkbox,
            nowrap_text_box,
            wrap_text_box,
        };
        ui.layout_controls();
        ui.set_line_wrap(false);

        Ok(ui)
    }

    fn layout_controls(&self) {
        let (width, height) = self.window.size();
        let content_width = width.saturating_sub(WINDOW_MARGIN * 2);

        self.file_label.set_size(content_width, LABEL_HEIGHT);
        self.details_label.set_size(content_width, LABEL_HEIGHT);
        self.wrap_checkbox.set_position(
            WINDOW_MARGIN as i32,
            (WINDOW_MARGIN + (LABEL_HEIGHT * 2) + LABEL_GAP) as i32,
        );
        self.wrap_checkbox.set_size(120, CHECKBOX_HEIGHT);

        let text_top =
            WINDOW_MARGIN + (LABEL_HEIGHT * 2) + LABEL_GAP + CHECKBOX_HEIGHT + TEXTBOX_GAP;
        let text_height = height.saturating_sub(text_top + WINDOW_MARGIN);

        self.nowrap_text_box
            .set_position(WINDOW_MARGIN as i32, text_top as i32);
        self.nowrap_text_box.set_size(content_width, text_height);

        self.wrap_text_box
            .set_position(WINDOW_MARGIN as i32, text_top as i32);
        self.wrap_text_box.set_size(content_width, text_height);
    }

    fn set_line_wrap(&self, enabled: bool) {
        self.wrap_text_box.set_visible(enabled);
        self.nowrap_text_box.set_visible(!enabled);

        if enabled {
            self.wrap_text_box.set_focus();
        } else {
            self.nowrap_text_box.set_focus();
        }
    }
}

fn main() {
    if let Err(err) = run() {
        let _ = nwg::init();
        nwg::simple_message("read-first", &err);
    }
}

fn run() -> Result<(), String> {
    let (mode, file_path) = parse_args(env::args().skip(1).collect())?;

    if !Path::new(&file_path).exists() {
        return Err(format!("File not found: {file_path}"));
    }

    let result = read_for_display(&file_path, mode)?;

    nwg::init().map_err(|e| format!("Failed to initialize UI: {e}"))?;

    let ui = ViewerUi::build(mode, &file_path, result)
        .map_err(|e| format!("Failed to build UI: {e}"))?;

    let app = Rc::new(ui);
    let app_events = app.clone();
    let handler = Rc::new(RefCell::new(None));
    let handler_ref = handler.clone();

    *handler.borrow_mut() = Some(nwg::full_bind_event_handler(
        &app.window.handle,
        move |evt, _, handle| {
            if evt == nwg::Event::OnWindowClose && handle == app_events.window {
                nwg::stop_thread_dispatch();
            }

            if evt == nwg::Event::OnResize && handle == app_events.window {
                app_events.layout_controls();
            }

            if evt == nwg::Event::OnButtonClick && handle == app_events.wrap_checkbox {
                app_events.set_line_wrap(
                    app_events.wrap_checkbox.check_state() == nwg::CheckBoxState::Checked,
                );
            }
        },
    ));

    nwg::dispatch_thread_events();

    if let Some(h) = handler_ref.borrow_mut().take() {
        nwg::unbind_event_handler(&h);
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(ReadMode, String), String> {
    if args.len() != 3 || args[0] != "--mode" {
        return Err("Usage: read-first.exe --mode <first-line|first-megabyte> <file path>".into());
    }

    let mode = match args[1].as_str() {
        "first-line" => ReadMode::FirstLine,
        "first-megabyte" => ReadMode::FirstMegabyte,
        _ => return Err("Invalid mode. Use --mode first-line or --mode first-megabyte.".into()),
    };

    Ok((mode, args[2].clone()))
}

fn read_for_display(file_path: &str, mode: ReadMode) -> Result<ReadResult, String> {
    let mut file = File::open(file_path).map_err(|e| format!("Cannot open file: {e}"))?;

    let (bytes, details) = match mode {
        ReadMode::FirstLine => {
            let bytes =
                read_first_line(&mut file).map_err(|e| format!("Failed to read file: {e}"))?;
            (
                bytes.clone(),
                format!("Showing first line ({} bytes)", bytes.len()),
            )
        }
        ReadMode::FirstMegabyte => {
            let bytes = read_up_to(&mut file, ONE_MEGABYTE)
                .map_err(|e| format!("Failed to read file: {e}"))?;
            (
                bytes.clone(),
                format!("Showing first megabyte ({} bytes)", bytes.len()),
            )
        }
    };

    Ok(ReadResult {
        details,
        text: bytes_to_display_text(&bytes),
    })
}

fn read_up_to(file: &mut File, max_bytes: usize) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![0_u8; max_bytes];
    let read = file.read(&mut buffer)?;
    buffer.truncate(read);
    Ok(buffer)
}

fn read_first_line(file: &mut File) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut one = [0_u8; 1];

    loop {
        let read = file.read(&mut one)?;
        if read == 0 {
            break;
        }

        let current = one[0];
        if current == b'\n' {
            break;
        }

        if current == b'\r' {
            let read_next = file.read(&mut one)?;
            if read_next == 0 || one[0] != b'\n' {
                // Keep behavior simple and stop on CR, mirroring CRLF handling.
            }
            break;
        }

        bytes.push(current);
    }

    Ok(bytes)
}

fn bytes_to_display_text(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if b == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 1;
            }
            out.push_str("\r\n");
            i += 1;
            continue;
        }

        if b == b'\n' {
            out.push_str("\r\n");
            i += 1;
            continue;
        }

        if b == b'\t' {
            out.push('\t');
            i += 1;
            continue;
        }

        if (0x20..=0x7e).contains(&b) {
            out.push(b as char);
            i += 1;
            continue;
        }

        out.push('<');
        out.push_str(&format!("{b:02X}"));
        out.push('>');
        i += 1;
    }

    out
}
