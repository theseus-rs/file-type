use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975669: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_669,
        source_type: SourceType::Wikidata,
        name: "BMF",
        extensions: &["bmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
