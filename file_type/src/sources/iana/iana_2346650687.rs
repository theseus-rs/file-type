use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2346650687: FileType = FileType {
    file_format: &FileFormat {
        id: 2_346_650_687,
        source_type: SourceType::Iana,
        name: "vnd.japannet-verification",
        extensions: &[],
        media_types: &["application/vnd.japannet-verification"],
        signatures: &[],
        related_formats: &[],
    },
};
