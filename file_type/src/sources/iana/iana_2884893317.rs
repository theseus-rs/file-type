use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2884893317: FileType = FileType {
    file_format: &FileFormat {
        id: 2_884_893_317,
        source_type: SourceType::Iana,
        name: "vnd.motorola.iprm",
        extensions: &[],
        media_types: &["application/vnd.motorola.iprm"],
        signatures: &[],
        related_formats: &[],
    },
};
