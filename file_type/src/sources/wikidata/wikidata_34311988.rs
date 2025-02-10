use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34311988: FileType = FileType {
    file_format: &FileFormat {
        id: 34_311_988,
        source_type: SourceType::Wikidata,
        name: "Shen script",
        extensions: &["shen"],
        media_types: &["application/x-shen", "text/x-shen"],
        signatures: &[],
        related_formats: &[],
    },
};
