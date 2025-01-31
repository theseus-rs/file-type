use crate::format::FileFormat;

pub(crate) const LINGUIST_75: FileFormat = FileFormat {
    id: 75,
    puid: "linguist/75",
    name: "Csound Score",
    extensions: &["sco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
