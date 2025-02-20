use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111284103: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_103,
        source_type: SourceType::Wikidata,
        name: "G.726 3-bit (24 kbps) ADPCM format data",
        extensions: &["g726-3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
