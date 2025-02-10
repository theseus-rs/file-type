use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_33515758: FileType = FileType {
    file_format: &FileFormat {
        id: 33_515_758,
        source_type: SourceType::Wikidata,
        name: "LAS 1.4 file format",
        extensions: &["las"],
        media_types: &["application/vnd.las"],
        signatures: &[],
        related_formats: &[],
    },
};
