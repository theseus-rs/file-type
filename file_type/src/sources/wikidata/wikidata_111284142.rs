use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111284142: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_142,
        source_type: SourceType::Wikidata,
        name: "G.726 5-bit (40 kbps) ADPCM format data",
        extensions: &["g726-5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
