use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87894240: FileType = FileType {
    file_format: &FileFormat {
        id: 87_894_240,
        source_type: SourceType::Wikidata,
        name: "Avery Label Pro Document 3",
        extensions: &["lpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
