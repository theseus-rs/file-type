use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1880924644: FileType = FileType {
    file_format: &FileFormat {
        id: 1_880_924_644,
        source_type: SourceType::Iana,
        name: "TSVCIS",
        extensions: &[],
        media_types: &["audio/TSVCIS"],
        signatures: &[],
        related_formats: &[],
    },
};
