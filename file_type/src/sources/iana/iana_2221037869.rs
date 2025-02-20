use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2221037869: FileType = FileType {
    file_format: &FileFormat {
        id: 2_221_037_869,
        source_type: SourceType::Iana,
        name: "encaprtp",
        extensions: &[],
        media_types: &["text/encaprtp"],
        signatures: &[],
        related_formats: &[],
    },
};
