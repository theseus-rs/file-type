use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61813289: FileType = FileType {
    file_format: &FileFormat {
        id: 61_813_289,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 4",
        extensions: &["wdb"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
