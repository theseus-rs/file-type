use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129643497: FileType = FileType {
    file_format: &FileFormat {
        id: 129_643_497,
        source_type: SourceType::Wikidata,
        name: "Icon file format",
        extensions: &["icon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
