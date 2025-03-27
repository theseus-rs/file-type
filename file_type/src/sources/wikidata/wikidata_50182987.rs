use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50182987: FileType = FileType {
    file_format: &FileFormat {
        id: 50_182_987,
        source_type: SourceType::Wikidata,
        name: "Interleaved ADX Audio Format",
        extensions: &["aix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
