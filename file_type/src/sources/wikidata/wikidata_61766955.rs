use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61766955: FileType = FileType {
    file_format: &FileFormat {
        id: 61_766_955,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 2.0a",
        extensions: &["wdb"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
