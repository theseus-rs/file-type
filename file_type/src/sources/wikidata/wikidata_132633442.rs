use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132633442: FileType = FileType {
    file_format: &FileFormat {
        id: 132_633_442,
        source_type: SourceType::Wikidata,
        name: "Modula-3 linker information file format",
        extensions: &["ax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
