use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4267139476: FileType = FileType {
    file_format: &FileFormat {
        id: 4_267_139_476,
        source_type: SourceType::Iana,
        name: "vnd.route66.link66+xml",
        extensions: &[],
        media_types: &["application/vnd.route66.link66+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
