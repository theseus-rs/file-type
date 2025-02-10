use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1808196427: FileType = FileType {
    file_format: &FileFormat {
        id: 1_808_196_427,
        source_type: SourceType::Iana,
        name: "vnd.keyman.kmx",
        extensions: &[],
        media_types: &["application/vnd.keyman.kmx"],
        signatures: &[],
        related_formats: &[],
    },
};
