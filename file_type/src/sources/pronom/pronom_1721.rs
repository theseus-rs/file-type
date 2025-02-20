use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1721: FileType = FileType {
    file_format: &FileFormat {
        id: 1_721,
        source_type: SourceType::Pronom,
        name: "ESRI ArcMap Document",
        extensions: &["mxd", "mxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
