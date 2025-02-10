use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3554138563: FileType = FileType {
    file_format: &FileFormat {
        id: 3_554_138_563,
        source_type: SourceType::Iana,
        name: "vnd.meridian-slingshot",
        extensions: &[],
        media_types: &["application/vnd.meridian-slingshot"],
        signatures: &[],
        related_formats: &[],
    },
};
