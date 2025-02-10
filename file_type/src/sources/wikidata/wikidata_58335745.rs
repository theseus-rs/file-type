use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58335745: FileType = FileType {
    file_format: &FileFormat {
        id: 58_335_745,
        source_type: SourceType::Wikidata,
        name: "Acrobat Catalog Cat File",
        extensions: &["cat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
