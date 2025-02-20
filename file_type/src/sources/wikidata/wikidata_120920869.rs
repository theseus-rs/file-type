use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120920869: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_869,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2007 data file",
        extensions: &["m16"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
