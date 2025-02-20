use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119138441: FileType = FileType {
    file_format: &FileFormat {
        id: 119_138_441,
        source_type: SourceType::Wikidata,
        name: "FreeHand Template 7",
        extensions: &["ft7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
