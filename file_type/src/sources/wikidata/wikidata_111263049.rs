use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263049: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_049,
        source_type: SourceType::Wikidata,
        name: "ActiveMovie streaming format",
        extensions: &["asf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
