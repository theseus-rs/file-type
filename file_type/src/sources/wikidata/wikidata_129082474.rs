use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129082474: FileType = FileType {
    file_format: &FileFormat {
        id: 129_082_474,
        source_type: SourceType::Wikidata,
        name: "Elixir script file",
        extensions: &["exs"],
        media_types: &["text/x-elixir"],
        signatures: &[],
        related_formats: &[],
    },
};
