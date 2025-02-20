use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47012439: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_439,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Document file format",
        extensions: &["bps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
