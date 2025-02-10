use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61766587: FileType = FileType {
    file_format: &FileFormat {
        id: 61_766_587,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows",
        extensions: &["wdb"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
