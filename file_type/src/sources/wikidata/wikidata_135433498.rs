use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135433498: FileType = FileType {
    file_format: &FileFormat {
        id: 135_433_498,
        source_type: SourceType::Wikidata,
        name: "Template Toolkit template file",
        extensions: &["tt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
