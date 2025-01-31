use crate::format::FileFormat;

pub(crate) const LINGUIST_210: FileFormat = FileFormat {
    id: 210,
    puid: "linguist/210",
    name: "Logtalk",
    extensions: &["lgt", "logtalk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
