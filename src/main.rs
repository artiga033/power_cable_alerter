use std::{io, thread, time::Duration};

use windows::{
    core::{Interface, HSTRING},
    Data::Xml::Dom::XmlElement,
    Foundation::EventHandler,
    System::Power::{PowerManager, PowerSupplyStatus},
    UI::Notifications::{ToastNotification, ToastNotificationManager, ToastTemplateType},
};
fn main() {
    PowerManager::PowerSupplyStatusChanged(&EventHandler::new(|_, _| {
        let s = PowerManager::PowerSupplyStatus()?;
        match s {
            PowerSupplyStatus::NotPresent => {
                toast(
                    "Power supply plugged off",
                    &get_file_uri(OFF_IMG).unwrap_or("".into()),
                )?;
            }
            PowerSupplyStatus::Inadequate => {
                toast(
                    "Power supply plugged in, but inadequate",
                    &get_file_uri(OFF_IMG).unwrap_or("".into()),
                )?;
            }
            PowerSupplyStatus::Adequate => {
                toast(
                    "Power supply plugged in",
                    &get_file_uri(ON_IMG).unwrap_or("".into()),
                )?;
            }
            _ => {}
        }
        Ok(())
    }))
    .expect("failed to register event handler");
    loop {
        // just sleep forever, and the winrt events can still be triggered and executed
        thread::sleep(Duration::MAX);
    }
}

fn toast<S>(text: S, image_uri: &HSTRING) -> windows::core::Result<()>
where
    S: ToString,
{
    let template = ToastTemplateType::ToastImageAndText01;
    let toast_xml = ToastNotificationManager::GetTemplateContent(template)?;

    let img_nodes = toast_xml.GetElementsByTagName(&HSTRING::from("image"))?;
    let img_node = img_nodes.Item(0)?.cast::<XmlElement>()?;
    img_node.SetAttribute(&"src".into(), image_uri)?;

    let text_nodes = toast_xml.GetElementsByTagName(&HSTRING::from("text"))?;
    text_nodes.Item(0)?.SetInnerText(&text.to_string().into())?;

    let toast = ToastNotification::CreateToastNotification(&toast_xml)?;
    ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(env!("CARGO_PKG_NAME")))?
        .Show(&toast)?;
    Ok(())
}

static ON_IMG: (&'static str, &'static [u8]) = ("assets/on.png", include_bytes!("assets/on.png"));
static OFF_IMG: (&'static str, &'static [u8]) =
    ("assets/off.png", include_bytes!("assets/off.png"));

fn get_file_uri(embedded_file: (&'static str, &'static [u8])) -> io::Result<HSTRING> {
    let tmp_dir = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
    std::fs::create_dir_all(&tmp_dir)?;
    let (name, file) = embedded_file;
    let path = tmp_dir.join(name);
    std::fs::create_dir_all(&path.parent().unwrap())?;
    if !path.exists() {
        std::fs::write(&path, file)?;
    }
    Ok(HSTRING::from(format!("file://{}", &path.to_string_lossy())))
}
