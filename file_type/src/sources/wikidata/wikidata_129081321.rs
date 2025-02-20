use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129081321: FileType = FileType {
    file_format: &FileFormat {
        id: 129_081_321,
        source_type: SourceType::Wikidata,
        name: "Elixir source code file",
        extensions: &["ex"],
        media_types: &["text/x-elixir"],
        signatures: &[],
        related_formats: &[],
    },
};
