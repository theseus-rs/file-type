use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_104: FileType = FileType {
    file_format: &FileFormat {
        id: 104,
        source_type: SourceType::Linguist,
        name: "Erlang",
        extensions: &[
            "app", "app.src", "erl", "es", "escript", "hrl", "xrl", "yrl",
        ],
        media_types: &["text/x-erlang"],
        signatures: &[],
        related_formats: &[],
    },
};
