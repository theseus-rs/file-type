use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2503186602: FileFormat = FileFormat {
    id: 2_503_186_602,
    source_type: SourceType::Iana,
    name: "vnd.cab-jscript",
    extensions: &[],
    media_types: &["application/vnd.cab-jscript"],
    internal_signatures: &[],
    related_formats: &[],
};
