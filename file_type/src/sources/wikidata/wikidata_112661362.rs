use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112661362: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_362,
        source_type: SourceType::Wikidata,
        name: "Motion Analysis TRC File",
        extensions: &["trc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
