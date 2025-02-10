use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1023933872: FileType = FileType {
    file_format: &FileFormat {
        id: 1_023_933_872,
        source_type: SourceType::Iana,
        name: "vnd.dolby.pl2",
        extensions: &[],
        media_types: &["audio/vnd.dolby.pl2"],
        signatures: &[],
        related_formats: &[],
    },
};
