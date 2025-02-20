use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966897: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_897,
        source_type: SourceType::Wikidata,
        name: "Hudson Entertainment System",
        extensions: &["hes"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
