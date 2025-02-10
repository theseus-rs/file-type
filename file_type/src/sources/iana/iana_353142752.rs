use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_353142752: FileType = FileType {
    file_format: &FileFormat {
        id: 353_142_752,
        source_type: SourceType::Iana,
        name: "media-policy-dataset+xml",
        extensions: &[],
        media_types: &["application/media-policy-dataset+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
