use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_483127336: FileType = FileType {
    file_format: &FileFormat {
        id: 483_127_336,
        source_type: SourceType::Iana,
        name: "mpeg",
        extensions: &[],
        media_types: &["audio/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
