use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8042553: FileType = FileType {
    file_format: &FileFormat {
        id: 8_042_553,
        source_type: SourceType::Wikidata,
        name: "XVL",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
