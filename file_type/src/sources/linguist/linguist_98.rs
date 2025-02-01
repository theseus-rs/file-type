use crate::format::FileFormat;

pub(crate) const LINGUIST_98: FileFormat = FileFormat {
    id: 98,
    puid: "linguist/98",
    name: "Ecere Projects",
    extensions: &["epj"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
