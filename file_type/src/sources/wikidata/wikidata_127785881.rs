use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127785881: FileType = FileType {
    file_format: &FileFormat {
        id: 127_785_881,
        source_type: SourceType::Wikidata,
        name: "Modula-2 file",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
