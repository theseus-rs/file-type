use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111284090: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_090,
        source_type: SourceType::Wikidata,
        name: "G.723 3-bit (24 kbps) ADPCM format data",
        extensions: &["g723-3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
