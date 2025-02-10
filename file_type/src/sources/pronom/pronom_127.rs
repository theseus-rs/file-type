use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_127: FileType = FileType {
    file_format: &FileFormat {
        id: 127,
        source_type: SourceType::Pronom,
        name: "Hewlett Packard Vector Graphic Plotter File",
        extensions: &["plt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
