use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48815175: FileType = FileType {
    file_format: &FileFormat {
        id: 48_815_175,
        source_type: SourceType::Wikidata,
        name: "Framework Database, version 2",
        extensions: &["fw", "fw2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
