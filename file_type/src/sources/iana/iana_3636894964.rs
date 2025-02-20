use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
