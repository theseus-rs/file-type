use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823203: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_203,
        source_type: SourceType::Wikidata,
        name: "Synalysis grammar file",
        extensions: &["grammar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
