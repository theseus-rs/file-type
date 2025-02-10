use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120965378: FileType = FileType {
    file_format: &FileFormat {
        id: 120_965_378,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money version 2 data",
        extensions: &["mn2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
