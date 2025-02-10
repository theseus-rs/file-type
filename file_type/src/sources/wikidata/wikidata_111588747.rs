use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111588747: FileType = FileType {
    file_format: &FileFormat {
        id: 111_588_747,
        source_type: SourceType::Wikidata,
        name: "Inspiration Software File",
        extensions: &["isf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
