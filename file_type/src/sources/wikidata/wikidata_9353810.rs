use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_9353810: FileType = FileType {
    file_format: &FileFormat {
        id: 9_353_810,
        source_type: SourceType::Wikidata,
        name: "Oracle Database Trace File",
        extensions: &["trc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
