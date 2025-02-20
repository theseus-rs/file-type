use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650534: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_534,
        source_type: SourceType::Wikidata,
        name: "PaintJet soft font",
        extensions: &["pjf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
