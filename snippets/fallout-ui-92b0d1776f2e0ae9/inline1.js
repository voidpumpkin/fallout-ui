export function notify(level, msg) {
    let notification = notifier.notify(level, msg);
    notification.element.addEventListener('click', () => {
        notification.destroy();
    })
}