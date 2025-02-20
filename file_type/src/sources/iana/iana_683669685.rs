use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_683669685: FileType = FileType {
    file_format: &FileFormat {
        id: 683_669_685,
        source_type: SourceType::Iana,
        name: "secevent+jwt",
        extensions: &[],
        media_types: &["application/secevent+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
