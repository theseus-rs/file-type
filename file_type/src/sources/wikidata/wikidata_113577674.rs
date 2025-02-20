use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113577674: FileType = FileType {
    file_format: &FileFormat {
        id: 113_577_674,
        source_type: SourceType::Wikidata,
        name: "Prassi PrimoDVD",
        extensions: &["gi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
