use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115806228: FileType = FileType {
    file_format: &FileFormat {
        id: 115_806_228,
        source_type: SourceType::Wikidata,
        name: "JWCC",
        extensions: &["jwcc"],
        media_types: &["application/jwcc"],
        signatures: &[],
        related_formats: &[],
    },
};
