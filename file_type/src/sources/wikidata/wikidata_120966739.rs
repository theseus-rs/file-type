use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120966739: FileType = FileType {
    file_format: &FileFormat {
        id: 120_966_739,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2001 data",
        extensions: &["mn9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
