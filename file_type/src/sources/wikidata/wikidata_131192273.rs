use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131192273: FileType = FileType {
    file_format: &FileFormat {
        id: 131_192_273,
        source_type: SourceType::Wikidata,
        name: "Tact source code file",
        extensions: &["tact"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
