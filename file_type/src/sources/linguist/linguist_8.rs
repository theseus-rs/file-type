use crate::format::FileFormat;

pub(crate) const LINGUIST_8: FileFormat = FileFormat {
    id: 8,
    puid: "linguist/8",
    name: "Classic ASP",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
