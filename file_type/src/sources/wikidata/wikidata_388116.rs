use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_388116: FileType = FileType {
    file_format: &FileFormat {
        id: 388_116,
        source_type: SourceType::Wikidata,
        name: "Electronic Design Interchange Format",
        extensions: &["edf", "edi", "edn", "edo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
