use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3540040203: FileFormat = FileFormat {
    id: 3_540_040_203,
    source_type: SourceType::Iana,
    name: "vnd.xmpie.cpkg",
    extensions: &[],
    media_types: &["application/vnd.xmpie.cpkg"],
    internal_signatures: &[],
    related_formats: &[],
};
