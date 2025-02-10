use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2018760936: FileType = FileType {
    file_format: &FileFormat {
        id: 2_018_760_936,
        source_type: SourceType::Iana,
        name: "sipfrag",
        extensions: &[],
        media_types: &["message/sipfrag"],
        signatures: &[],
        related_formats: &[],
    },
};
