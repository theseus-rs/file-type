use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61777964: FileType = FileType {
    file_format: &FileFormat {
        id: 61_777_964,
        source_type: SourceType::Wikidata,
        name: "Play SID Audio, version 2",
        extensions: &["psid", "sid"],
        media_types: &["audio/prs.sid"],
        signatures: &[],
        related_formats: &[],
    },
};
