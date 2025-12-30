use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25100386: FileType = FileType {
    file_format: &FileFormat {
        id: 25_100_386,
        source_type: SourceType::Wikidata,
        name: "NeXML",
        extensions: &["nexml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
