use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60615282: FileType = FileType {
    file_format: &FileFormat {
        id: 60_615_282,
        source_type: SourceType::Wikidata,
        name: "Write for Windows Document, version 3",
        extensions: &["wri"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
