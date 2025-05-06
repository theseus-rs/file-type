use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134292868: FileType = FileType {
    file_format: &FileFormat {
        id: 134_292_868,
        source_type: SourceType::Wikidata,
        name: "Clipper pre-linked library file",
        extensions: &["pll", "plt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
