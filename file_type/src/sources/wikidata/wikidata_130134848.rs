use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130134848: FileType = FileType {
    file_format: &FileFormat {
        id: 130_134_848,
        source_type: SourceType::Wikidata,
        name: "Kusto query file",
        extensions: &["kql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
