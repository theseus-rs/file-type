use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317350: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_350,
        source_type: SourceType::Wikidata,
        name: "Matlab variable binary file",
        extensions: &["mat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
