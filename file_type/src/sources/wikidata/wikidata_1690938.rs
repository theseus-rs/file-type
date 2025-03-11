use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1690938: FileType = FileType {
    file_format: &FileFormat {
        id: 1_690_938,
        source_type: SourceType::Wikidata,
        name: "Job Definition Format",
        extensions: &[],
        media_types: &["application/vnd.cip4-jdf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
