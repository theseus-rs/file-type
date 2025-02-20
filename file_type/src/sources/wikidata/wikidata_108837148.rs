use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
