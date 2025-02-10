use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4274586742: FileType = FileType {
    file_format: &FileFormat {
        id: 4_274_586_742,
        source_type: SourceType::Iana,
        name: "vnd.nokia.conml+wbxml",
        extensions: &[],
        media_types: &["application/vnd.nokia.conml+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
