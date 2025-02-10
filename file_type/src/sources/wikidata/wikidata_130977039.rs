use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130977039: FileType = FileType {
    file_format: &FileFormat {
        id: 130_977_039,
        source_type: SourceType::Wikidata,
        name: "Singularity definition file",
        extensions: &["Singularity", "def"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
