use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122829936: FileType = FileType {
    file_format: &FileFormat {
        id: 122_829_936,
        source_type: SourceType::Wikidata,
        name: "Creativity Workshop PWK file",
        extensions: &["pwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
