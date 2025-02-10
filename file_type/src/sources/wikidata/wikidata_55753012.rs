use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55753012: FileType = FileType {
    file_format: &FileFormat {
        id: 55_753_012,
        source_type: SourceType::Wikidata,
        name: "Microsoft xWMA file format",
        extensions: &["xwma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
