use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51801668: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_668,
        source_type: SourceType::Wikidata,
        name: "MS-DOS Text File with line breaks",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
