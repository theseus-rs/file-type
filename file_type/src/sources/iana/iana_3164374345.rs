use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3164374345: FileType = FileType {
    file_format: &FileFormat {
        id: 3_164_374_345,
        source_type: SourceType::Iana,
        name: "vnd.3gpp-prose-pc3ach+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp-prose-pc3ach+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
