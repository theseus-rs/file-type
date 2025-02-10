use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1798121: FileType = FileType {
    file_format: &FileFormat {
        id: 1_798_121,
        source_type: SourceType::Wikidata,
        name: "Microsoft Library",
        extensions: &["lib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
