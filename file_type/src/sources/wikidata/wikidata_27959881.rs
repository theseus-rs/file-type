use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959881: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_881,
        source_type: SourceType::Wikidata,
        name: "Propellerhead Reason NN-XT Patch File",
        extensions: &["sx2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
