use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67206676: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_676,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for Windows Drawing file",
        extensions: &["tcw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
