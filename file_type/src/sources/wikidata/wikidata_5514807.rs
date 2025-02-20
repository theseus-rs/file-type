use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5514807: FileType = FileType {
    file_format: &FileFormat {
        id: 5_514_807,
        source_type: SourceType::Wikidata,
        name: "GUIDO music notation",
        extensions: &["gmn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
