use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116647633: FileType = FileType {
    file_format: &FileFormat {
        id: 116_647_633,
        source_type: SourceType::Wikidata,
        name: "KeyForm 3.x Document",
        extensions: &["ifd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
