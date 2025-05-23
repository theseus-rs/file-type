use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_355: FileType = FileType {
    file_format: &FileFormat {
        id: 355,
        source_type: SourceType::Pronom,
        name: "Microsoft FoxPro Database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
