use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47486880: FileType = FileType {
    file_format: &FileFormat {
        id: 47_486_880,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Catalog (Unix)",
        extensions: &["sas7bcat", "sc7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
