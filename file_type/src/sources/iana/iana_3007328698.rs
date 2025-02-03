use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3007328698: FileFormat = FileFormat {
    id: 3_007_328_698,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite.wem",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite.wem"],
    internal_signatures: &[],
    related_formats: &[],
};
