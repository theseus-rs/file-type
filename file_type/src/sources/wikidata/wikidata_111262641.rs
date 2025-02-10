use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111262641: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_641,
        source_type: SourceType::Wikidata,
        name: "Muon DS404 bank file",
        extensions: &["404"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
