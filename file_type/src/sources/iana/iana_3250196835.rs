use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3250196835: FileType = FileType {
    file_format: &FileFormat {
        id: 3_250_196_835,
        source_type: SourceType::Iana,
        name: "ATRAC3",
        extensions: &[],
        media_types: &["audio/ATRAC3"],
        signatures: &[],
        related_formats: &[],
    },
};
