use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4070898466: FileType = FileType {
    file_format: &FileFormat {
        id: 4_070_898_466,
        source_type: SourceType::Iana,
        name: "vnd.mozilla.apng",
        extensions: &[],
        media_types: &["image/vnd.mozilla.apng"],
        signatures: &[],
        related_formats: &[],
    },
};
