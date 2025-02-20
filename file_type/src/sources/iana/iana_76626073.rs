use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_76626073: FileType = FileType {
    file_format: &FileFormat {
        id: 76_626_073,
        source_type: SourceType::Iana,
        name: "rpki-updown",
        extensions: &[],
        media_types: &["application/rpki-updown"],
        signatures: &[],
        related_formats: &[],
    },
};
