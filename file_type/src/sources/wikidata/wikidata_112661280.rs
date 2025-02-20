use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112661280: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_280,
        source_type: SourceType::Wikidata,
        name: "Lightscape View file",
        extensions: &["vw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
