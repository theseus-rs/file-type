use crate::format::FileFormat;

pub(crate) const LINGUIST_5: FileFormat = FileFormat {
    id: 5,
    puid: "linguist/5",
    name: "API Blueprint",
    extensions: &["apib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
