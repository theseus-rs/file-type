use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10376670: FileType = FileType {
    file_format: &FileFormat {
        id: 10_376_670,
        source_type: SourceType::Wikidata,
        name: "tar.bz2",
        extensions: &["tar.bz2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
