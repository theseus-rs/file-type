use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2866723734: FileType = FileType {
    file_format: &FileFormat {
        id: 2_866_723_734,
        source_type: SourceType::Iana,
        name: "byteranges",
        extensions: &[],
        media_types: &["multipart/byteranges"],
        signatures: &[],
        related_formats: &[],
    },
};
