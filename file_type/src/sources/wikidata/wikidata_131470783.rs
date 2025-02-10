use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131470783: FileType = FileType {
    file_format: &FileFormat {
        id: 131_470_783,
        source_type: SourceType::Wikidata,
        name: "MetaImage Source file",
        extensions: &["mha"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
