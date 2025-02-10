use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1105824498: FileType = FileType {
    file_format: &FileFormat {
        id: 1_105_824_498,
        source_type: SourceType::Iana,
        name: "3gpp-ims+xml",
        extensions: &[],
        media_types: &["application/3gpp-ims+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
