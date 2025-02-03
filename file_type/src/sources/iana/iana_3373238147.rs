use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3373238147: FileFormat = FileFormat {
    id: 3_373_238_147,
    source_type: SourceType::Iana,
    name: "vnd.1000minds.decision-model+xml",
    extensions: &[],
    media_types: &["application/vnd.1000minds.decision-model+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
