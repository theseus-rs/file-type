use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127817534: FileType = FileType {
    file_format: &FileFormat {
        id: 127_817_534,
        source_type: SourceType::Wikidata,
        name: "gp script",
        extensions: &["gp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
