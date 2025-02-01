use crate::format::FileFormat;

pub(crate) const LINGUIST_19: FileFormat = FileFormat {
    id: 19,
    puid: "linguist/19",
    name: "AppleScript",
    extensions: &["applescript", "scpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
