use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
