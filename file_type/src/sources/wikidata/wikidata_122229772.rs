use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122229772: FileType = FileType {
    file_format: &FileFormat {
        id: 122_229_772,
        source_type: SourceType::Wikidata,
        name: "Digital Interface Format",
        extensions: &["dif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
