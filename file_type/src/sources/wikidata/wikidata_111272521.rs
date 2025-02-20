use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272521: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_521,
        source_type: SourceType::Wikidata,
        name: "Ensoniq VFX-SD instrument file",
        extensions: &["efv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
