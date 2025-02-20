use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_268086: FileType = FileType {
    file_format: &FileFormat {
        id: 268_086,
        source_type: SourceType::Wikidata,
        name: "OMDoc",
        extensions: &["omdoc"],
        media_types: &["application/omdoc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
