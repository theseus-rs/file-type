use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1246631980: FileFormat = FileFormat {
    id: 1_246_631_980,
    source_type: SourceType::Iana,
    name: "vnd.fdsn.seed",
    extensions: &[],
    media_types: &["application/vnd.fdsn.seed"],
    internal_signatures: &[],
    related_formats: &[],
};
