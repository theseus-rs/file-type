use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61777776: FileType = FileType {
    file_format: &FileFormat {
        id: 61_777_776,
        source_type: SourceType::Wikidata,
        name: "Play SID Audio, version 1",
        extensions: &["dxr", "psid"],
        media_types: &["audio/prs.sid"],
        signatures: &[],
        related_formats: &[],
    },
};
