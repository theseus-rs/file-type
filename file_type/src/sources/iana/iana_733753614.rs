use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_733753614: FileType = FileType {
    file_format: &FileFormat {
        id: 733_753_614,
        source_type: SourceType::Iana,
        name: "vnd.trid.tpt",
        extensions: &[],
        media_types: &["application/vnd.trid.tpt"],
        signatures: &[],
        related_formats: &[],
    },
};
