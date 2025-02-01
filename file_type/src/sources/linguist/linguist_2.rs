use crate::format::FileFormat;

pub(crate) const LINGUIST_2: FileFormat = FileFormat {
    id: 2,
    puid: "linguist/2",
    name: "AGS Script",
    extensions: &["asc", "ash"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
