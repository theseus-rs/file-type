use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_269: FileType = FileType {
    file_format: &FileFormat {
        id: 269,
        source_type: SourceType::Pronom,
        name: "NeXt Sound",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
