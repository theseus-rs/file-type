use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119519641: FileType = FileType {
    file_format: &FileFormat {
        id: 119_519_641,
        source_type: SourceType::Wikidata,
        name: "Windows Spelling Dictionary Identifier",
        extensions: &["dub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
