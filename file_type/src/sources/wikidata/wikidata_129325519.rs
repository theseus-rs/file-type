use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129325519: FileType = FileType {
    file_format: &FileFormat {
        id: 129_325_519,
        source_type: SourceType::Wikidata,
        name: "FunC file format",
        extensions: &["fc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
