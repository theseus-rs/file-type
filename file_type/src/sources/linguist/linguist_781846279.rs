use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_781846279: FileType = FileType {
    file_format: &FileFormat {
        id: 781_846_279,
        source_type: SourceType::Linguist,
        name: "X PixMap",
        extensions: &["pm", "xpm"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
