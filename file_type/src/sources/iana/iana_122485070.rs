use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_122485070: FileType = FileType {
    file_format: &FileFormat {
        id: 122_485_070,
        source_type: SourceType::Iana,
        name: "scip",
        extensions: &[],
        media_types: &["audio/scip"],
        signatures: &[],
        related_formats: &[],
    },
};
