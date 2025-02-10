use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2478575557: FileType = FileType {
    file_format: &FileFormat {
        id: 2_478_575_557,
        source_type: SourceType::Iana,
        name: "vnd.ibm.MiniPay",
        extensions: &[],
        media_types: &["application/vnd.ibm.MiniPay"],
        signatures: &[],
        related_formats: &[],
    },
};
