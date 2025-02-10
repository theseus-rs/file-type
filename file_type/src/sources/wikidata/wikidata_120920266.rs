use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120920266: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_266,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money version 1 data",
        extensions: &["mn1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
