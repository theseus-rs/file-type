use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207350: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_350,
        source_type: SourceType::Wikidata,
        name: "Video Display Adapter",
        extensions: &["vda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
