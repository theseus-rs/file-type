use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2582906171: FileType = FileType {
    file_format: &FileFormat {
        id: 2_582_906_171,
        source_type: SourceType::Iana,
        name: "cql-expression",
        extensions: &[],
        media_types: &["text/cql-expression"],
        signatures: &[],
        related_formats: &[],
    },
};
