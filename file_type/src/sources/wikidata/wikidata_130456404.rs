use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130456404: FileType = FileType {
    file_format: &FileFormat {
        id: 130_456_404,
        source_type: SourceType::Wikidata,
        name: "Beancount fileformat",
        extensions: &["beancount"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
