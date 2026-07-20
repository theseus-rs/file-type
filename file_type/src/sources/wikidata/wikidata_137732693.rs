use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137732693: FileType = FileType {
    file_format: &FileFormat {
        id: 137_732_693,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate video capture file",
        extensions: &["cpvc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
