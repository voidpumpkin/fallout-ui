use std::error::Error;

use wasm_bindgen::prelude::wasm_bindgen;

use super::web_error::WebError;
use super::web_error::WebErrorKind;

#[wasm_bindgen(inline_js = "
export function init_notifier(show_time) {
    let sharedClasses = 'opacity-80 hover:opacity-100 cursor-pointer rounded-sm border-transparent';
    window.notifier = new Notifier({
        default_time: show_time,
        success: {
            classes: `${sharedClasses} bg-success`,
            textColor: 'white',
            progressColor: 'white',
            icon: `\
<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 20 20\" fill=\"white\" class=\"!w-6 !h-6\">\
    <path fill-rule=\"evenodd\" d=\"M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z\" clip-rule=\"evenodd\" />\
</svg>`
        },
        error: {
            classes: `${sharedClasses} bg-danger`,
            textColor: 'white',
            progressColor: 'white',
            icon: `\
<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 20 20\" fill=\"white\" class=\"!w-6 !h-6\">\
    <path fill-rule=\"evenodd\" d=\"M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z\" clip-rule=\"evenodd\" />\
</svg>`
        },
        warning: {
            classes: `${sharedClasses} bg-primary`,
            textColor: 'white',
            progressColor: 'white',
            icon: `\
<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 20 20\" fill=\"white\" class=\"!w-6 !h-6\">\
    <path \
        fill-rule=\"evenodd\" \
        d=\"M8.485 2.495c.673-1.167 2.357-1.167 3.03 0l6.28 10.875c.673 1.167-.17 2.625-1.516 2.625H3.72c-1.347 0-2.189-1.458-1.515-2.625L8.485 2.495zM10 5a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5A.75.75 0 0110 5zm0 9a1 1 0 100-2 1 1 0 000 2z\" \
        clip-rule=\"evenodd\" \
    />\
</svg>`
        },
        info: {
            classes: `${sharedClasses} bg-secondary`,
            textColor: 'white',
            progressColor: 'white',
            icon: `\
<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 20 20\" fill=\"white\" class=\"!w-6 !h-6\">\
    <path \
        fill-rule=\"evenodd\" \
        d=\"M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z\" \
        clip-rule=\"evenodd\" \
    />\
</svg>`
        }
    });
}")]
extern "C" {
    pub fn init_notifier(show_time: u32);
}

#[wasm_bindgen(inline_js = "export function notify(level, msg) {
    let notification = notifier.notify(level, msg);
    notification.element.addEventListener('click', () => {
        notification.destroy();
    })
}")]
extern "C" {
    pub fn notify(level: &str, msg: &str);
}

pub fn notify_error(msg: &str) {
    notify("error", msg);
}

pub fn notify_warning(msg: &str) {
    notify("warning", msg);
}

pub fn notify_success(msg: &str) {
    notify("success", msg);
}

pub fn notify_info(msg: &str) {
    notify("info", msg);
}

pub fn notify_err<T>(err: T)
where
    T: Error + Into<NotificationMsg>,
{
    log::error!("{:#?}", err);
    let NotificationMsg { message, level } = err.into();
    notify(level.as_str(), message.as_str());
}

pub struct NotificationMsg {
    message: String,
    level: String,
}

impl From<WebError> for NotificationMsg {
    fn from(err: WebError) -> Self {
        NotificationMsg {
            message: format!("{}", err),
            level: match err.kind {
                WebErrorKind::Validation(_) => "warning".to_string(),
                WebErrorKind::JsError(_) | WebErrorKind::LogicError => "error".to_string(),
            },
        }
    }
}
