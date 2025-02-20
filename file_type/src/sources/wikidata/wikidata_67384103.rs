use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67384103: FileType = FileType {
    file_format: &FileFormat {
        id: 67_384_103,
        source_type: SourceType::Wikidata,
        name: "ArtMoney Table File",
        extensions: &["amt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
