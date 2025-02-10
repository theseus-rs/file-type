use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105849830: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_830,
        source_type: SourceType::Wikidata,
        name: "Camtasia Studio Screen Recording",
        extensions: &["camrec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
