use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111283922: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_922,
        source_type: SourceType::Wikidata,
        name: "ITU G.722 8-bit (64 kbps) ADPCM format data",
        extensions: &["g722-8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
