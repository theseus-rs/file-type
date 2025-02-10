use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4246575942: FileType = FileType {
    file_format: &FileFormat {
        id: 4_246_575_942,
        source_type: SourceType::Iana,
        name: "vnd.ms-playready.media.pyv",
        extensions: &[],
        media_types: &["video/vnd.ms-playready.media.pyv"],
        signatures: &[],
        related_formats: &[],
    },
};
