use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959836: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_836,
        source_type: SourceType::Wikidata,
        name: "Raw FL Studio Project",
        extensions: &["flp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
