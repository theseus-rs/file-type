use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4215538110: FileType = FileType {
    file_format: &FileFormat {
        id: 4_215_538_110,
        source_type: SourceType::Iana,
        name: "fastinfoset",
        extensions: &[],
        media_types: &["application/fastinfoset"],
        signatures: &[],
        related_formats: &[],
    },
};
