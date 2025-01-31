use crate::format::FileFormat;

pub(crate) const LINGUIST_412: FileFormat = FileFormat {
    id: 412,
    puid: "linguist/412",
    name: "desktop",
    extensions: &["desktop", "desktop.in", "service"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
