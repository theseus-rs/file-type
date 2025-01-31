use crate::format::FileFormat;

pub(crate) const LINGUIST_194: FileFormat = FileFormat {
    id: 194,
    puid: "linguist/194",
    name: "LabVIEW",
    extensions: &["lvclass", "lvlib", "lvproj"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
