use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2678541341: FileType = FileType {
    file_format: &FileFormat {
        id: 2_678_541_341,
        source_type: SourceType::Iana,
        name: "vnd.tml",
        extensions: &[],
        media_types: &["application/vnd.tml"],
        signatures: &[],
        related_formats: &[],
    },
};
