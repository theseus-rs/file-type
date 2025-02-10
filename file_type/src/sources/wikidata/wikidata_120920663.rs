use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120920663: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_663,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2003 data",
        extensions: &["m11"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
