use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_96929146: FileType = FileType {
    file_format: &FileFormat {
        id: 96_929_146,
        source_type: SourceType::Iana,
        name: "vnd.iptvforum.1dparityfec-2005",
        extensions: &[],
        media_types: &["video/vnd.iptvforum.1dparityfec-2005"],
        signatures: &[],
        related_formats: &[],
    },
};
