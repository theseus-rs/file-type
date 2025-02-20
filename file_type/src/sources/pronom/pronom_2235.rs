use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2235: FileType = FileType {
    file_format: &FileFormat {
        id: 2_235,
        source_type: SourceType::Pronom,
        name: "Corel Print House Document",
        extensions: &["cph", "cpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
