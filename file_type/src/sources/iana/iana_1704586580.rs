use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1704586580: FileType = FileType {
    file_format: &FileFormat {
        id: 1_704_586_580,
        source_type: SourceType::Iana,
        name: "vnd.cendio.thinlinc.clientconf",
        extensions: &[],
        media_types: &["application/vnd.cendio.thinlinc.clientconf"],
        signatures: &[],
        related_formats: &[],
    },
};
