use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111341823: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_823,
        source_type: SourceType::Wikidata,
        name: "Signed dwords (32-bit) data",
        extensions: &["sdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
