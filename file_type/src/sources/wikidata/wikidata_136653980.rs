use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136653980: FileType = FileType {
    file_format: &FileFormat {
        id: 136_653_980,
        source_type: SourceType::Wikidata,
        name: "Raku script file",
        extensions: &["raku"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
