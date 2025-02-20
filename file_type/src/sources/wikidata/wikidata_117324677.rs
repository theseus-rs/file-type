use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117324677: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_677,
        source_type: SourceType::Wikidata,
        name: "User Interface file",
        extensions: &["uir"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
