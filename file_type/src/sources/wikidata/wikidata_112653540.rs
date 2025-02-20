use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112653540: FileType = FileType {
    file_format: &FileFormat {
        id: 112_653_540,
        source_type: SourceType::Wikidata,
        name: "WebScan Files",
        extensions: &["wsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
