use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113674366: FileType = FileType {
    file_format: &FileFormat {
        id: 113_674_366,
        source_type: SourceType::Wikidata,
        name: "Final Draft 5-7 Template",
        extensions: &["fdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
