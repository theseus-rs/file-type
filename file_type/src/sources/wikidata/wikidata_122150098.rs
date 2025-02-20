use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122150098: FileType = FileType {
    file_format: &FileFormat {
        id: 122_150_098,
        source_type: SourceType::Wikidata,
        name: "TaxAct 2008 Tax Return Backup File",
        extensions: &["ba8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
