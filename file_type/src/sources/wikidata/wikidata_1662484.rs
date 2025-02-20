use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1662484: FileType = FileType {
    file_format: &FileFormat {
        id: 1_662_484,
        source_type: SourceType::Wikidata,
        name: "Information Presentation Facility",
        extensions: &["inf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
