use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2178: FileType = FileType {
    file_format: &FileFormat {
        id: 2_178,
        source_type: SourceType::Pronom,
        name: "Picture Publisher Bitmap",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
