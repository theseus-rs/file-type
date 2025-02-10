use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122150082: FileType = FileType {
    file_format: &FileFormat {
        id: 122_150_082,
        source_type: SourceType::Wikidata,
        name: "TaxAct 2008 Tax Return File",
        extensions: &["ta8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
