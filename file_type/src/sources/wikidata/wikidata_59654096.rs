use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59654096: FileType = FileType {
    file_format: &FileFormat {
        id: 59_654_096,
        source_type: SourceType::Wikidata,
        name: "Adobe Content Server Message File",
        extensions: &["acsm"],
        media_types: &["application/vnd.adobe.adept+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
