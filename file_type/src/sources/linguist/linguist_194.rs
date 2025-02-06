use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_194: FileFormat = FileFormat {
    id: 194,
    source_type: SourceType::Linguist,
    name: "LabVIEW",
    extensions: &["lvclass", "lvlib", "lvproj"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
