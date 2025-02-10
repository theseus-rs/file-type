use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131395745: FileType = FileType {
    file_format: &FileFormat {
        id: 131_395_745,
        source_type: SourceType::Wikidata,
        name: "VGL source code file",
        extensions: &["rpf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
