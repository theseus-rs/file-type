use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_267956: FileType = FileType {
    file_format: &FileFormat {
        id: 267_956,
        source_type: SourceType::Wikidata,
        name: "Atom",
        extensions: &["atom"],
        media_types: &["application/atom+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
