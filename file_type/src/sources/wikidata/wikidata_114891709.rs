use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114891709: FileType = FileType {
    file_format: &FileFormat {
        id: 114_891_709,
        source_type: SourceType::Wikidata,
        name: "Tax Export File",
        extensions: &["txf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
