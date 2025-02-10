use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3636894964: FileType = FileType {
    file_format: &FileFormat {
        id: 3_636_894_964,
        source_type: SourceType::Iana,
        name: "vnd.graphviz",
        extensions: &[],
        media_types: &["text/vnd.graphviz"],
        signatures: &[],
        related_formats: &[],
    },
};
