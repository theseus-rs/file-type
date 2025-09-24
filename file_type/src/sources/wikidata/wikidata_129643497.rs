use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129643497: FileType = FileType {
    file_format: &FileFormat {
        id: 129_643_497,
        source_type: SourceType::Wikidata,
        name: "Icon source code file",
        extensions: &["icon"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
