use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342780: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_780,
        source_type: SourceType::Wikidata,
        name: "Propellerheads Reason 2 NN-XT format",
        extensions: &["sxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
