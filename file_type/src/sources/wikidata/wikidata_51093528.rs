use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51093528: FileType = FileType {
    file_format: &FileFormat {
        id: 51_093_528,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Pattern",
        extensions: &["pat"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
