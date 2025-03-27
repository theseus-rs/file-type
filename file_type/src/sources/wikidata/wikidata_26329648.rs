use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26329648: FileType = FileType {
    file_format: &FileFormat {
        id: 26_329_648,
        source_type: SourceType::Wikidata,
        name: "JPEG File Interchange Format (JFIF), version 1.0",
        extensions: &["jpeg", "jpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
