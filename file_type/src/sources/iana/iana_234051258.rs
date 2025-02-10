use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_234051258: FileType = FileType {
    file_format: &FileFormat {
        id: 234_051_258,
        source_type: SourceType::Iana,
        name: "raptorfec",
        extensions: &[],
        media_types: &["text/raptorfec"],
        signatures: &[],
        related_formats: &[],
    },
};
