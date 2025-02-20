use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_76622680: FileType = FileType {
    file_format: &FileFormat {
        id: 76_622_680,
        source_type: SourceType::Wikidata,
        name: "Turboprint Wizard",
        extensions: &["wizard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
