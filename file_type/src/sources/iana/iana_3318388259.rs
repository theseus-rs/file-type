use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3318388259: FileType = FileType {
    file_format: &FileFormat {
        id: 3_318_388_259,
        source_type: SourceType::Iana,
        name: "rtploopback",
        extensions: &[],
        media_types: &["text/rtploopback"],
        signatures: &[],
        related_formats: &[],
    },
};
