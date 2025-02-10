use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109996995: FileType = FileType {
    file_format: &FileFormat {
        id: 109_996_995,
        source_type: SourceType::Wikidata,
        name: "OrgPlus Template",
        extensions: &["opxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
