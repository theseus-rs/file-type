use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2396751051: FileFormat = FileFormat {
    id: 2_396_751_051,
    source_type: SourceType::Iana,
    name: "smpte336m",
    extensions: &[],
    media_types: &["application/smpte336m"],
    internal_signatures: &[],
    related_formats: &[],
};
