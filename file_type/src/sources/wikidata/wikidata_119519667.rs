use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119519667: FileType = FileType {
    file_format: &FileFormat {
        id: 119_519_667,
        source_type: SourceType::Wikidata,
        name: "DubIt Project",
        extensions: &["dub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
