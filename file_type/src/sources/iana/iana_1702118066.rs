use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1702118066: FileType = FileType {
    file_format: &FileFormat {
        id: 1_702_118_066,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.catmetadata+xml",
        extensions: &[],
        media_types: &["application/vnd.gentoo.catmetadata+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
