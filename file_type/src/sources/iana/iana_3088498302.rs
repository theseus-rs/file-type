use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3088498302: FileType = FileType {
    file_format: &FileFormat {
        id: 3_088_498_302,
        source_type: SourceType::Iana,
        name: "L20",
        extensions: &[],
        media_types: &["audio/L20"],
        signatures: &[],
        related_formats: &[],
    },
};
