use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551372: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_372,
        source_type: SourceType::Wikidata,
        name: "Adobe Monitor Setup File",
        extensions: &["ams"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
