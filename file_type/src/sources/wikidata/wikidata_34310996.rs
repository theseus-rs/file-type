use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34310996: FileType = FileType {
    file_format: &FileFormat {
        id: 34_310_996,
        source_type: SourceType::Wikidata,
        name: "Security Catalog",
        extensions: &["cat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
