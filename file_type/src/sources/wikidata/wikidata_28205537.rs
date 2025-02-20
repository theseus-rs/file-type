use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205537: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_537,
        source_type: SourceType::Wikidata,
        name: "Micrografx Icon",
        extensions: &["icn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
