
fn main() -> windows_registry::Result<()> {
    let key = windows_registry::CURRENT_USER.create(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize")?;

    if key.get_u32("AppsUseLightTheme")? == 0 {
        key.set_u32("AppsUseLightTheme", 1)?;
    } else {
        key.set_u32("AppsUseLightTheme", 0)?;
    }

    Ok(())
}
