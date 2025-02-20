use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131288311: FileType = FileType {
    file_format: &FileFormat {
        id: 131_288_311,
        source_type: SourceType::Wikidata,
        name: "Transaction Execution Approval Language file format",
        extensions: &["teal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
