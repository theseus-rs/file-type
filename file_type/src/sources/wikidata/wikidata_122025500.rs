use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122025500: FileType = FileType {
    file_format: &FileFormat {
        id: 122_025_500,
        source_type: SourceType::Wikidata,
        name: "Scorch web page",
        extensions: &["htm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
