use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_779014290: FileType = FileType {
    file_format: &FileFormat {
        id: 779_014_290,
        source_type: SourceType::Iana,
        name: "vnd.japannet-registration-wakeup",
        extensions: &[],
        media_types: &["application/vnd.japannet-registration-wakeup"],
        signatures: &[],
        related_formats: &[],
    },
};
