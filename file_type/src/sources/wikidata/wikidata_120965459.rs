use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120965459: FileType = FileType {
    file_format: &FileFormat {
        id: 120_965_459,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money version 3 data",
        extensions: &["mn3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
