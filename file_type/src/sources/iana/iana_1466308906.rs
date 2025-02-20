use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1466308906: FileType = FileType {
    file_format: &FileFormat {
        id: 1_466_308_906,
        source_type: SourceType::Iana,
        name: "grammar-ref-list",
        extensions: &[],
        media_types: &["text/grammar-ref-list"],
        signatures: &[],
        related_formats: &[],
    },
};
