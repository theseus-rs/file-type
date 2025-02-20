use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2204: FileType = FileType {
    file_format: &FileFormat {
        id: 2_204,
        source_type: SourceType::Pronom,
        name: "Muvee autoProducer Project File",
        extensions: &["mve"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
