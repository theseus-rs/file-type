use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2944600447: FileType = FileType {
    file_format: &FileFormat {
        id: 2_944_600_447,
        source_type: SourceType::Iana,
        name: "wgsl",
        extensions: &[],
        media_types: &["text/wgsl"],
        signatures: &[],
        related_formats: &[],
    },
};
