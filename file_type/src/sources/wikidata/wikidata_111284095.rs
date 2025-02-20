use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111284095: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_095,
        source_type: SourceType::Wikidata,
        name: "G.723 5-bit (40 kbps) ADPCM format data",
        extensions: &["g723-5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
