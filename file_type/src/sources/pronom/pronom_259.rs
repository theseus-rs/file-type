use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_259: FileType = FileType {
    file_format: &FileFormat {
        id: 259,
        source_type: SourceType::Pronom,
        name: "Silicon Graphics RGB File",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
