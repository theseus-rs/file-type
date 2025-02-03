use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1080111604: FileFormat = FileFormat {
    id: 1_080_111_604,
    source_type: SourceType::Iana,
    name: "vnd.otps.ct-kip+xml",
    extensions: &[],
    media_types: &["application/vnd.otps.ct-kip+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
