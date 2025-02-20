use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_72724699: FileType = FileType {
    file_format: &FileFormat {
        id: 72_724_699,
        source_type: SourceType::Wikidata,
        name: "Newsletters And More document",
        extensions: &["nam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
