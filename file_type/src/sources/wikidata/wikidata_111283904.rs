use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111283904: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_904,
        source_type: SourceType::Wikidata,
        name: "ITU G.722 6-bit (48 kbps) ADPCM format data",
        extensions: &["g722-6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
