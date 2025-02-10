use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111315905: FileType = FileType {
    file_format: &FileFormat {
        id: 111_315_905,
        source_type: SourceType::Wikidata,
        name: "IBM MWave DSP synthesizer bank setup",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
