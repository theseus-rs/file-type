use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111009850: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_850,
        source_type: SourceType::Wikidata,
        name: "PrintMaster T-Shirt File format",
        extensions: &["tsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
