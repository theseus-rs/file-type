use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133538700: FileType = FileType {
    file_format: &FileFormat {
        id: 133_538_700,
        source_type: SourceType::Wikidata,
        name: "DataShow Screen file",
        extensions: &["scr"],
        media_types: &["image/x-datashow-screen"],
        signatures: &[],
        related_formats: &[],
    },
};
