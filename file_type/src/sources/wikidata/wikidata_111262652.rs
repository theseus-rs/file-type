use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111262652: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_652,
        source_type: SourceType::Wikidata,
        name: "Muon DS404 patch file",
        extensions: &["404"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
