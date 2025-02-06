use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1717841530: FileFormat = FileFormat {
    id: 1_717_841_530,
    source_type: SourceType::Iana,
    name: "tamp-error",
    extensions: &[],
    media_types: &["application/tamp-error"],
    signatures: &[],
    related_formats: &[],
};
