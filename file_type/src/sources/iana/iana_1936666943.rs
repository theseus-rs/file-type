use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1936666943: FileType = FileType {
    file_format: &FileFormat {
        id: 1_936_666_943,
        source_type: SourceType::Iana,
        name: "vnd.kodak-descriptor",
        extensions: &[],
        media_types: &["application/vnd.kodak-descriptor"],
        signatures: &[],
        related_formats: &[],
    },
};
