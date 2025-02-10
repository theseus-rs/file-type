use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063276: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_276,
        source_type: SourceType::Wikidata,
        name: "SAP Document",
        extensions: &["ali"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
