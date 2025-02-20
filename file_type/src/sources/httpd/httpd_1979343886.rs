use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1979343886: FileType = FileType {
    file_format: &FileFormat {
        id: 1_979_343_886,
        source_type: SourceType::Httpd,
        name: "mp4",
        extensions: &["mp4s"],
        media_types: &["application/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
