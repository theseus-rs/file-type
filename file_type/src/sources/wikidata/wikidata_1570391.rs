use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1570391: FileType = FileType {
    file_format: &FileFormat {
        id: 1_570_391,
        source_type: SourceType::Wikidata,
        name: "Uuencoding",
        extensions: &["uu", "uue"],
        media_types: &["text/x-uuencode"],
        signatures: &[],
        related_formats: &[],
    },
};
