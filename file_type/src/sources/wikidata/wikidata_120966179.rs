use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120966179: FileType = FileType {
    file_format: &FileFormat {
        id: 120_966_179,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 98 data file",
        extensions: &["mn6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
