use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272667: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_667,
        source_type: SourceType::Wikidata,
        name: "Logic EXS24 instrument file",
        extensions: &["exs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
