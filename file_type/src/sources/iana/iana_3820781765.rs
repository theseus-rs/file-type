use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3820781765: FileType = FileType {
    file_format: &FileFormat {
        id: 3_820_781_765,
        source_type: SourceType::Iana,
        name: "shex",
        extensions: &[],
        media_types: &["text/shex"],
        signatures: &[],
        related_formats: &[],
    },
};
