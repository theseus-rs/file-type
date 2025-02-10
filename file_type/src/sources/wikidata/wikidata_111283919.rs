use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111283919: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_919,
        source_type: SourceType::Wikidata,
        name: "ITU G.722 7-bit (56 kbps) ADPCM format data",
        extensions: &["g722-7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
