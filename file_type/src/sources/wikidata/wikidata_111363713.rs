use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363713: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_713,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XF 'waveforms' format",
        extensions: &["x3w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
