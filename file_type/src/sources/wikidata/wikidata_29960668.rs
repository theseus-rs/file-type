use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29960668: FileType = FileType {
    file_format: &FileFormat {
        id: 29_960_668,
        source_type: SourceType::Wikidata,
        name: "RenderWare binary stream file",
        extensions: &["dff", "txd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
