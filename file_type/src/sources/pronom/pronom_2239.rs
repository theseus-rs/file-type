use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2239: FileType = FileType {
    file_format: &FileFormat {
        id: 2_239,
        source_type: SourceType::Pronom,
        name: "Corel Print House/Print Office Document",
        extensions: &["cph", "cpd", "cpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
