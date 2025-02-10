use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125347324: FileType = FileType {
    file_format: &FileFormat {
        id: 125_347_324,
        source_type: SourceType::Wikidata,
        name: "Binary Grid File",
        extensions: &["grb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
