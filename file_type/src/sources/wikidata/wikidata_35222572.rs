use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_35222572: FileType = FileType {
    file_format: &FileFormat {
        id: 35_222_572,
        source_type: SourceType::Wikidata,
        name: "RAR, version 3",
        extensions: &["rar"],
        media_types: &["application/vnd.rar", "application/x-rar-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
