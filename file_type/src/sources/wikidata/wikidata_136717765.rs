use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136717765: FileType = FileType {
    file_format: &FileFormat {
        id: 136_717_765,
        source_type: SourceType::Wikidata,
        name: "Redcode RAW (R3D) Media File 1",
        extensions: &["r3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
