use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113276129: FileType = FileType {
    file_format: &FileFormat {
        id: 113_276_129,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Photo Pages",
        extensions: &["pho"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
