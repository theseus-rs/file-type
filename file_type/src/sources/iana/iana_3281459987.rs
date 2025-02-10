use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3281459987: FileType = FileType {
    file_format: &FileFormat {
        id: 3_281_459_987,
        source_type: SourceType::Iana,
        name: "ief",
        extensions: &[],
        media_types: &["image/ief"],
        signatures: &[],
        related_formats: &[],
    },
};
