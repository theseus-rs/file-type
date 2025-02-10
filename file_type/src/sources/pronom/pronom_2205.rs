use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2205: FileType = FileType {
    file_format: &FileFormat {
        id: 2_205,
        source_type: SourceType::Pronom,
        name: "Muvee autoProducer Project File",
        extensions: &["mvex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
