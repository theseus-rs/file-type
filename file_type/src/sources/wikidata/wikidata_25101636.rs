use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25101636: FileType = FileType {
    file_format: &FileFormat {
        id: 25_101_636,
        source_type: SourceType::Wikidata,
        name: "IVUE",
        extensions: &["ivue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
