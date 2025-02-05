use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1063909402: FileFormat = FileFormat {
    id: 1_063_909_402,
    source_type: SourceType::Iana,
    name: "vnd.criticaltools.wbs+xml",
    extensions: &[],
    media_types: &["application/vnd.criticaltools.wbs+xml"],
    signatures: &[],
    related_formats: &[],
};
