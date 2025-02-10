use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2980547604: FileType = FileType {
    file_format: &FileFormat {
        id: 2_980_547_604,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.alert",
        extensions: &[],
        media_types: &["application/vnd.uplanet.alert"],
        signatures: &[],
        related_formats: &[],
    },
};
