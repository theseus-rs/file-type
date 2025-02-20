use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52426198: FileType = FileType {
    file_format: &FileFormat {
        id: 52_426_198,
        source_type: SourceType::Wikidata,
        name: "XYWrite for Windows Document, version 4",
        extensions: &["xy4", "xyw"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
