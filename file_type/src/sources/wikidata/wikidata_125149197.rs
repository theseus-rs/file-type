use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125149197: FileType = FileType {
    file_format: &FileFormat {
        id: 125_149_197,
        source_type: SourceType::Wikidata,
        name: "Units Data File",
        extensions: &["units"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
