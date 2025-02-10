use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59468329: FileType = FileType {
    file_format: &FileFormat {
        id: 59_468_329,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System Data XPT (Unix)",
        extensions: &["xpt"],
        media_types: &["application/x-sas-xport"],
        signatures: &[],
        related_formats: &[],
    },
};
