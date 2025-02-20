use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115116023: FileType = FileType {
    file_format: &FileFormat {
        id: 115_116_023,
        source_type: SourceType::Wikidata,
        name: "Funpaint Image File",
        extensions: &["fp2", "fun", "vic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
