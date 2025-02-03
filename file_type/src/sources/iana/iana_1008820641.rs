use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1008820641: FileFormat = FileFormat {
    id: 1_008_820_641,
    source_type: SourceType::Iana,
    name: "PCMU-WB",
    extensions: &[],
    media_types: &["audio/PCMU-WB"],
    internal_signatures: &[],
    related_formats: &[],
};
