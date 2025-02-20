use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3111506699: FileType = FileType {
    file_format: &FileFormat {
        id: 3_111_506_699,
        source_type: SourceType::Iana,
        name: "vnd.iptvforum.1dparityfec-1010",
        extensions: &[],
        media_types: &["video/vnd.iptvforum.1dparityfec-1010"],
        signatures: &[],
        related_formats: &[],
    },
};
