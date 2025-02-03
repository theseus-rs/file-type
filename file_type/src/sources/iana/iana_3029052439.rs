use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3029052439: FileFormat = FileFormat {
    id: 3_029_052_439,
    source_type: SourceType::Iana,
    name: "eac3",
    extensions: &[],
    media_types: &["audio/eac3"],
    internal_signatures: &[],
    related_formats: &[],
};
