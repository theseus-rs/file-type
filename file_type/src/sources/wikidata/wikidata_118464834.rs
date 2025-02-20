use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118464834: FileType = FileType {
    file_format: &FileFormat {
        id: 118_464_834,
        source_type: SourceType::Wikidata,
        name: "Enhanced Image Package",
        extensions: &["eip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
