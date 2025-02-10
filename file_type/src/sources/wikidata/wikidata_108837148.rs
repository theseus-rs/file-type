use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108837148: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_148,
        source_type: SourceType::Wikidata,
        name: "Quicken Data Backup",
        extensions: &["qdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
