use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122047541: FileType = FileType {
    file_format: &FileFormat {
        id: 122_047_541,
        source_type: SourceType::Wikidata,
        name: "cc:Mail Archive Format",
        extensions: &["cca"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
