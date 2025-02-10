use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49243748: FileType = FileType {
    file_format: &FileFormat {
        id: 49_243_748,
        source_type: SourceType::Wikidata,
        name: "Acrobat Language definition file",
        extensions: &["lng"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
