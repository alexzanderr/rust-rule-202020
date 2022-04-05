
use core_dev::linuxapi::linux_notification_from_image_buffer;
use core_dev::imagelib::create_image_buffer_from_bytes;
use color_backtrace::install;



fn main() {
    install();

    static NOTIFICATION_ICON: &[u8] = include_bytes!("../../static/icons/rule-202020.png");

    let image_buffer = create_image_buffer_from_bytes(NOTIFICATION_ICON);

    linux_notification_from_image_buffer(
        "its working",
        "i've sent a notification from image loaded into the binary file of the app",
        &image_buffer,
        1);
    linux_notification_from_image_buffer(
        "its working",
        "i've sent a notification from image loaded into the binary file of the app",
        &image_buffer,
        1);
    linux_notification_from_image_buffer(
        "its working",
        "i've sent a notification from image loaded into the binary file of the app",
        &image_buffer,
        10);
}
