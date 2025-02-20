use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2184: FileType = FileType {
    file_format: &FileFormat {
        id: 2_184,
        source_type: SourceType::Pronom,
        name: "ESRI Published Map Format",
        extensions: &["pmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
