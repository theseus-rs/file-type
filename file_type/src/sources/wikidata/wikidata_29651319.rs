use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29651319: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_319,
        source_type: SourceType::Wikidata,
        name: "PixieScript",
        extensions: &["pxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
