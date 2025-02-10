use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113501237: FileType = FileType {
    file_format: &FileFormat {
        id: 113_501_237,
        source_type: SourceType::Wikidata,
        name: "Time Stamp Token",
        extensions: &["tst"],
        media_types: &["application/vnd.etsi.timestamp-token"],
        signatures: &[],
        related_formats: &[],
    },
};
