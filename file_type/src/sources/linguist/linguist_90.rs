use crate::format::FileFormat;

pub(crate) const LINGUIST_90: FileFormat = FileFormat {
    id: 90,
    puid: "linguist/90",
    name: "Dogescript",
    extensions: &["djs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
