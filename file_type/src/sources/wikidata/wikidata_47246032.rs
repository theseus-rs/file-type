use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47246032: FileType = FileType {
    file_format: &FileFormat {
        id: 47_246_032,
        source_type: SourceType::Wikidata,
        name: "PowerVR Object Data",
        extensions: &["pod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
