use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109944440: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_440,
        source_type: SourceType::Wikidata,
        name: "CadKey file format",
        extensions: &["prt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
