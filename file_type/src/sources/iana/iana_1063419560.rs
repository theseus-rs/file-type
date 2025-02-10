use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1063419560: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_419_560,
        source_type: SourceType::Iana,
        name: "vnd.ims.lis.v2.result+json",
        extensions: &[],
        media_types: &["application/vnd.ims.lis.v2.result+json"],
        signatures: &[],
        related_formats: &[],
    },
};
