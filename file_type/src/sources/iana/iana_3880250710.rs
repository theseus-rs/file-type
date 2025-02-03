use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3880250710: FileFormat = FileFormat {
    id: 3_880_250_710,
    source_type: SourceType::Iana,
    name: "vnd.shade-save-file",
    extensions: &[],
    media_types: &["application/vnd.shade-save-file"],
    internal_signatures: &[],
    related_formats: &[],
};
