use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_204: FileType = FileType {
    file_format: &FileFormat {
        id: 204,
        source_type: SourceType::Pronom,
        name: "OS/2 Change Control File",
        extensions: &["cin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
