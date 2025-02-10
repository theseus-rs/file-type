use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28771316: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_316,
        source_type: SourceType::Wikidata,
        name: "Micrografx Draw",
        extensions: &["drw"],
        media_types: &["application/x-mgx-designer"],
        signatures: &[],
        related_formats: &[],
    },
};
