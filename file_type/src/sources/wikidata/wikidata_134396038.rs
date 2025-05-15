use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134396038: FileType = FileType {
    file_format: &FileFormat {
        id: 134_396_038,
        source_type: SourceType::Wikidata,
        name: "TreeDraw chart",
        extensions: &["tdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
