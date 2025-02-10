use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66220018: FileType = FileType {
    file_format: &FileFormat {
        id: 66_220_018,
        source_type: SourceType::Wikidata,
        name: "Adobe GoLive Actions file format",
        extensions: &["actions"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
