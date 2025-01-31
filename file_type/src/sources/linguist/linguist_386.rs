use crate::format::FileFormat;

pub(crate) const LINGUIST_386: FileFormat = FileFormat {
    id: 386,
    puid: "linguist/386",
    name: "Vala",
    extensions: &["vala", "vapi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
