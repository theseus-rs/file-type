use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363569: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_569,
        source_type: SourceType::Wikidata,
        name: "id Software Music Format (700Hz)",
        extensions: &["wlf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
