use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111283902: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_902,
        source_type: SourceType::Wikidata,
        name: "ITU G.722 ADPCM format data",
        extensions: &["g722"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
