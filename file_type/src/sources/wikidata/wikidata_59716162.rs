use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59716162: FileType = FileType {
    file_format: &FileFormat {
        id: 59_716_162,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Chart",
        extensions: &["ch3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
