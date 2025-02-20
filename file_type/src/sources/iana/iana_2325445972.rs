use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2325445972: FileType = FileType {
    file_format: &FileFormat {
        id: 2_325_445_972,
        source_type: SourceType::Iana,
        name: "QCELP",
        extensions: &[],
        media_types: &["audio/QCELP"],
        signatures: &[],
        related_formats: &[],
    },
};
