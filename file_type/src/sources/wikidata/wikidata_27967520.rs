use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967520: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_520,
        source_type: SourceType::Wikidata,
        name: "Matroska 3D Stereoscopic video",
        extensions: &["mk3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
