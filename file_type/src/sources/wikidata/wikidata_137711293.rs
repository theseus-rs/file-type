use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137711293: FileType = FileType {
    file_format: &FileFormat {
        id: 137_711_293,
        source_type: SourceType::Wikidata,
        name: "OmegaT glossary file",
        extensions: &["txt", "utf8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
