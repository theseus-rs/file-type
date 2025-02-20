use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116884493: FileType = FileType {
    file_format: &FileFormat {
        id: 116_884_493,
        source_type: SourceType::Wikidata,
        name: "EPS Tiff Preview",
        extensions: &["eps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
