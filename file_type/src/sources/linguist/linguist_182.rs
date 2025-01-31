use crate::format::FileFormat;

pub(crate) const LINGUIST_182: FileFormat = FileFormat {
    id: 182,
    puid: "linguist/182",
    name: "Java Server Pages",
    extensions: &["jsp", "tag"],
    media_types: &["application/x-jsp"],
    internal_signatures: &[],
    related_formats: &[],
};
