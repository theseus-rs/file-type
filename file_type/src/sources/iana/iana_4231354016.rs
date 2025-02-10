use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4231354016: FileType = FileType {
    file_format: &FileFormat {
        id: 4_231_354_016,
        source_type: SourceType::Iana,
        name: "32kadpcm",
        extensions: &[],
        media_types: &["audio/32kadpcm"],
        signatures: &[],
        related_formats: &[],
    },
};
