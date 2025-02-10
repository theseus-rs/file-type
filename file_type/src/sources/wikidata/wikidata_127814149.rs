use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127814149: FileType = FileType {
    file_format: &FileFormat {
        id: 127_814_149,
        source_type: SourceType::Wikidata,
        name: "Ox source code file",
        extensions: &["ox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
