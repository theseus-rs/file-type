use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2699: FileType = FileType {
    file_format: &FileFormat {
        id: 2_699,
        source_type: SourceType::Pronom,
        name: "Esri ArcMap Label file",
        extensions: &["lxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
