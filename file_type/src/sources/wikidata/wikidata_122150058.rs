use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122150058: FileType = FileType {
    file_format: &FileFormat {
        id: 122_150_058,
        source_type: SourceType::Wikidata,
        name: "TaxAct 2007 Tax Return File",
        extensions: &["ta7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
