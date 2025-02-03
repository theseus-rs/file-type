use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1393458892: FileFormat = FileFormat {
    id: 1_393_458_892,
    source_type: SourceType::Iana,
    name: "vnd.logipipe.circuit+zip",
    extensions: &[],
    media_types: &["application/vnd.logipipe.circuit+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
