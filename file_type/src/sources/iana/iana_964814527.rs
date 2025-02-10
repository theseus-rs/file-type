use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_964814527: FileType = FileType {
    file_format: &FileFormat {
        id: 964_814_527,
        source_type: SourceType::Iana,
        name: "strings",
        extensions: &[],
        media_types: &["text/strings"],
        signatures: &[],
        related_formats: &[],
    },
};
