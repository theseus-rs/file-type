use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116969050: FileType = FileType {
    file_format: &FileFormat {
        id: 116_969_050,
        source_type: SourceType::Wikidata,
        name: "RS Red Storm File",
        extensions: &["rsb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
