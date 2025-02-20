use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120920280: FileType = FileType {
    file_format: &FileFormat {
        id: 120_920_280,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money 2002 data",
        extensions: &["m10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
