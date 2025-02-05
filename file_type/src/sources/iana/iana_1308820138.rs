use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1308820138: FileFormat = FileFormat {
    id: 1_308_820_138,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite.fis",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite.fis"],
    signatures: &[],
    related_formats: &[],
};
