use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3915112689: FileFormat = FileFormat {
    id: 3_915_112_689,
    source_type: SourceType::Iana,
    name: "vnd.swiftview-ics",
    extensions: &[],
    media_types: &["application/vnd.swiftview-ics"],
    signatures: &[],
    related_formats: &[],
};
