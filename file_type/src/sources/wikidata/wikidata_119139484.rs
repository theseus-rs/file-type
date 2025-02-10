use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119139484: FileType = FileType {
    file_format: &FileFormat {
        id: 119_139_484,
        source_type: SourceType::Wikidata,
        name: "FreeHand Template 8",
        extensions: &["ft8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
