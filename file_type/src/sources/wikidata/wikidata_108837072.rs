use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108837072: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_072,
        source_type: SourceType::Wikidata,
        name: "Nero HFS-CD Compilation",
        extensions: &["nhf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
