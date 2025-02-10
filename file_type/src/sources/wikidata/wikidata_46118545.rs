use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_46118545: FileType = FileType {
    file_format: &FileFormat {
        id: 46_118_545,
        source_type: SourceType::Wikidata,
        name: "Lotus Approach View File",
        extensions: &["apt"],
        media_types: &["application/vnd.lotus-approach"],
        signatures: &[],
        related_formats: &[],
    },
};
