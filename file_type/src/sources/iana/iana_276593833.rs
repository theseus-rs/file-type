use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_276593833: FileType = FileType {
    file_format: &FileFormat {
        id: 276_593_833,
        source_type: SourceType::Iana,
        name: "vnd.truedoc",
        extensions: &[],
        media_types: &["application/vnd.truedoc"],
        signatures: &[],
        related_formats: &[],
    },
};
