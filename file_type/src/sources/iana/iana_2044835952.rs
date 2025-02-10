use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2044835952: FileType = FileType {
    file_format: &FileFormat {
        id: 2_044_835_952,
        source_type: SourceType::Iana,
        name: "raptorfec",
        extensions: &[],
        media_types: &["audio/raptorfec"],
        signatures: &[],
        related_formats: &[],
    },
};
