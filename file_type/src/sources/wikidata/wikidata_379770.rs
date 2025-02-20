use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_379770: FileType = FileType {
    file_format: &FileFormat {
        id: 379_770,
        source_type: SourceType::Wikidata,
        name: "AVCHD",
        extensions: &["avchd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
