use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206446: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_446,
        source_type: SourceType::Wikidata,
        name: "KiSS CEL 4-bit",
        extensions: &["cel"],
        media_types: &["image/x-kiss-cel"],
        signatures: &[],
        related_formats: &[],
    },
};
