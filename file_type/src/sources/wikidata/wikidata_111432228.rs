use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111432228: FileType = FileType {
    file_format: &FileFormat {
        id: 111_432_228,
        source_type: SourceType::Wikidata,
        name: "HTTP File Server Template",
        extensions: &["tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
