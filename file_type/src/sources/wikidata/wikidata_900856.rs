use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_900856: FileType = FileType {
    file_format: &FileFormat {
        id: 900_856,
        source_type: SourceType::Wikidata,
        name: "manifest file",
        extensions: &["manifest"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
