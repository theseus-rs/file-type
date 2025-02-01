use crate::format::FileFormat;

pub(crate) const LINGUIST_405: FileFormat = FileFormat {
    id: 405,
    puid: "linguist/405",
    name: "Xojo",
    extensions: &[
        "xojo_code",
        "xojo_menu",
        "xojo_report",
        "xojo_script",
        "xojo_toolbar",
        "xojo_window",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
