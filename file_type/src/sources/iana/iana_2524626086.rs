use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2524626086: FileType = FileType {
    file_format: &FileFormat {
        id: 2_524_626_086,
        source_type: SourceType::Iana,
        name: "suit-report+cose",
        extensions: &[],
        media_types: &["application/suit-report+cose"],
        signatures: &[],
        related_formats: &[],
    },
};
