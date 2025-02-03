use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2931975007: FileFormat = FileFormat {
    id: 2_931_975_007,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite.ttc",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite.ttc"],
    internal_signatures: &[],
    related_formats: &[],
};
