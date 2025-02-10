use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2996336798: FileType = FileType {
    file_format: &FileFormat {
        id: 2_996_336_798,
        source_type: SourceType::Iana,
        name: "vnd.lotus-screencam",
        extensions: &[],
        media_types: &["application/vnd.lotus-screencam"],
        signatures: &[],
        related_formats: &[],
    },
};
