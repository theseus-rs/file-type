use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2831585141: FileType = FileType {
    file_format: &FileFormat {
        id: 2_831_585_141,
        source_type: SourceType::Iana,
        name: "mads+xml",
        extensions: &[],
        media_types: &["application/mads+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
