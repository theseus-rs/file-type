use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134596947: FileType = FileType {
    file_format: &FileFormat {
        id: 134_596_947,
        source_type: SourceType::Wikidata,
        name: "Pyspread compressed file",
        extensions: &["pys"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
