use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131471298: FileType = FileType {
    file_format: &FileFormat {
        id: 131_471_298,
        source_type: SourceType::Wikidata,
        name: "MGH file format",
        extensions: &["mgh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
