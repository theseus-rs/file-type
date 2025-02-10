use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61811585: FileType = FileType {
    file_format: &FileFormat {
        id: 61_811_585,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 3.0b",
        extensions: &["wdb"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
