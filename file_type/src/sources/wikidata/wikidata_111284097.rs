use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111284097: FileType = FileType {
    file_format: &FileFormat {
        id: 111_284_097,
        source_type: SourceType::Wikidata,
        name: "G.726 2/3/4/5-bit ADPCM format data",
        extensions: &["g726"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
