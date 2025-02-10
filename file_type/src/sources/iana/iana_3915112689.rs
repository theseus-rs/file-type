use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3915112689: FileType = FileType {
    file_format: &FileFormat {
        id: 3_915_112_689,
        source_type: SourceType::Iana,
        name: "vnd.swiftview-ics",
        extensions: &[],
        media_types: &["application/vnd.swiftview-ics"],
        signatures: &[],
        related_formats: &[],
    },
};
