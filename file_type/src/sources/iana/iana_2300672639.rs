use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2300672639: FileType = FileType {
    file_format: &FileFormat {
        id: 2_300_672_639,
        source_type: SourceType::Iana,
        name: "vnd.iptvforum.2dparityfec-1010",
        extensions: &[],
        media_types: &["video/vnd.iptvforum.2dparityfec-1010"],
        signatures: &[],
        related_formats: &[],
    },
};
