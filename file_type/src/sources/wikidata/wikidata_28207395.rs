use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207395: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_395,
        source_type: SourceType::Wikidata,
        name: "Texas Instruments picture file",
        extensions: &[
            "73i", "82i", "83i", "85i", "86i", "89i", "8xi", "92i", "9xi", "v2i",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
