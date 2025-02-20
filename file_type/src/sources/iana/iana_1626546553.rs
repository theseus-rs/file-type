use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1626546553: FileType = FileType {
    file_format: &FileFormat {
        id: 1_626_546_553,
        source_type: SourceType::Iana,
        name: "vnd.isac.fcs",
        extensions: &[],
        media_types: &["application/vnd.isac.fcs"],
        signatures: &[],
        related_formats: &[],
    },
};
