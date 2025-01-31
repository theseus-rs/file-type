use crate::format::FileFormat;

pub(crate) const LINGUIST_96: FileFormat = FileFormat {
    id: 96,
    puid: "linguist/96",
    name: "EQ",
    extensions: &["eq"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
