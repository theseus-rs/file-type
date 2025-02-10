use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975892: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_892,
        source_type: SourceType::Wikidata,
        name: "Force Control File",
        extensions: &["frc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
