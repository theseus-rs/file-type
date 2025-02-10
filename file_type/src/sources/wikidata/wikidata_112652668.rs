use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112652668: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_668,
        source_type: SourceType::Wikidata,
        name: "Gold Disk Anim format",
        extensions: &["awm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
