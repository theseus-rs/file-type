use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128693639: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_639,
        source_type: SourceType::Wikidata,
        name: "BBC Basic file",
        extensions: &["bbc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
