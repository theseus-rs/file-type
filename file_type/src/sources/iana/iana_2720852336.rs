use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2720852336: FileType = FileType {
    file_format: &FileFormat {
        id: 2_720_852_336,
        source_type: SourceType::Iana,
        name: "vnd.radgamettools.bink",
        extensions: &[],
        media_types: &["video/vnd.radgamettools.bink"],
        signatures: &[],
        related_formats: &[],
    },
};
