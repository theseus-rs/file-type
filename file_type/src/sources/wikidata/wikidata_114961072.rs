use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114961072: FileType = FileType {
    file_format: &FileFormat {
        id: 114_961_072,
        source_type: SourceType::Wikidata,
        name: "Writer's DreamKit 4.0 file",
        extensions: &["dsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
