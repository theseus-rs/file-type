use crate::format::FileFormat;

pub(crate) const LINGUIST_94: FileFormat = FileFormat {
    id: 94,
    puid: "linguist/94",
    name: "ECLiPSe",
    extensions: &["ecl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
