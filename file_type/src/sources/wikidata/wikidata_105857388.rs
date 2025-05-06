use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105857388: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_388,
        source_type: SourceType::Wikidata,
        name: "JSON Playlist File",
        extensions: &["jspf"],
        media_types: &["text/json"],
        signatures: &[],
        related_formats: &[],
    },
};
