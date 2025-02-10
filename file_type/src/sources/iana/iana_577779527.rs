use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_577779527: FileType = FileType {
    file_format: &FileFormat {
        id: 577_779_527,
        source_type: SourceType::Iana,
        name: "bacnet-xdd+zip",
        extensions: &[],
        media_types: &["application/bacnet-xdd+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
