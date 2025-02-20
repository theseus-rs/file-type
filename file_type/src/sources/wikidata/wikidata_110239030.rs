use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110239030: FileType = FileType {
    file_format: &FileFormat {
        id: 110_239_030,
        source_type: SourceType::Wikidata,
        name: "FrameForge 3D Studio",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
