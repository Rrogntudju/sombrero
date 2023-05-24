use registry::{Data, Error, Hive, Security};

fn main() -> Result<(), Error> {
    let themes = Hive::CurrentUser.open(
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
        Security::QueryValue | Security::SetValue,
    )?;

    if matches!(themes.value("AppsUseLightTheme")?, Data::U32(0)) {
        themes.set_value("AppsUseLightTheme", &Data::U32(1))?;
    } else {
        themes.set_value("AppsUseLightTheme", &Data::U32(0))?;
    }

    Ok(())
}
