use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61777675: FileType = FileType {
    file_format: &FileFormat {
        id: 61_777_675,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 3a",
        extensions: &["wdb"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
