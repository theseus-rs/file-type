use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110455242: FileType = FileType {
    file_format: &FileFormat {
        id: 110_455_242,
        source_type: SourceType::Wikidata,
        name: "Septentrio Binary Format",
        extensions: &["sbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
