use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_241995952: FileType = FileType {
    file_format: &FileFormat {
        id: 241_995_952,
        source_type: SourceType::Iana,
        name: "vnd.onepagertamp",
        extensions: &[],
        media_types: &["application/vnd.onepagertamp"],
        signatures: &[],
        related_formats: &[],
    },
};
