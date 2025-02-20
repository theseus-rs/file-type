use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120920758: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_758,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2005 backup data",
        extensions: &["m14"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
