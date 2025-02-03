use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2070717028: FileFormat = FileFormat {
    id: 2_070_717_028,
    source_type: SourceType::Iana,
    name: "javascript",
    extensions: &[],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
