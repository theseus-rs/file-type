use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51842171: FileType = FileType {
    file_format: &FileFormat {
        id: 51_842_171,
        source_type: SourceType::Wikidata,
        name: "MacPaint Graphics",
        extensions: &["pnt"],
        media_types: &["image/mac"],
        signatures: &[],
        related_formats: &[],
    },
};
