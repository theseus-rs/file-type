use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132633477: FileType = FileType {
    file_format: &FileFormat {
        id: 132_633_477,
        source_type: SourceType::Wikidata,
        name: "Modula-3 base program file format",
        extensions: &["b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
