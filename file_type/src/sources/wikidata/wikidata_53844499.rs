use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_53844499: FileType = FileType {
    file_format: &FileFormat {
        id: 53_844_499,
        source_type: SourceType::Wikidata,
        name: "BibTeX style file",
        extensions: &["bst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
