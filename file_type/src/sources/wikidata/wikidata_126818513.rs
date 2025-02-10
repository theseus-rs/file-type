use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126818513: FileType = FileType {
    file_format: &FileFormat {
        id: 126_818_513,
        source_type: SourceType::Wikidata,
        name: "Erlang source code file",
        extensions: &["erl"],
        media_types: &["text/x-erlang"],
        signatures: &[],
        related_formats: &[],
    },
};
