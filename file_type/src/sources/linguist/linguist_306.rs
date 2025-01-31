use crate::format::FileFormat;

pub(crate) const LINGUIST_306: FileFormat = FileFormat {
    id: 306,
    puid: "linguist/306",
    name: "QMake",
    extensions: &["pri", "pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
