use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1793: FileType = FileType {
    file_format: &FileFormat {
        id: 1_793,
        source_type: SourceType::Pronom,
        name: "ESRI ArcScene Document",
        extensions: &["sxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
