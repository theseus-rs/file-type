use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111363816: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_816,
        source_type: SourceType::Wikidata,
        name: "Raw Yamaha 4-bit ADPCM format data",
        extensions: &["yadpcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
