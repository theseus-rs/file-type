use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3391896951: FileFormat = FileFormat {
    id: 3_391_896_951,
    source_type: SourceType::Iana,
    name: "CPIM",
    extensions: &[],
    media_types: &["message/CPIM"],
    internal_signatures: &[],
    related_formats: &[],
};
