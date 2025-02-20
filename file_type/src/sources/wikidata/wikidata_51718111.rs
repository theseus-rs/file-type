use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51718111: FileType = FileType {
    file_format: &FileFormat {
        id: 51_718_111,
        source_type: SourceType::Wikidata,
        name: "AutoCAD ACIS Export File",
        extensions: &["sat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
