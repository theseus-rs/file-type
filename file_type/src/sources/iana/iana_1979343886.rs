use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1979343886: FileType = FileType {
    file_format: &FileFormat {
        id: 1_979_343_886,
        source_type: SourceType::Iana,
        name: "mp4",
        extensions: &[],
        media_types: &["application/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
