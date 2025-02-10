use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3523163259: FileType = FileType {
    file_format: &FileFormat {
        id: 3_523_163_259,
        source_type: SourceType::Iana,
        name: "vnd.insors.igm",
        extensions: &[],
        media_types: &["application/vnd.insors.igm"],
        signatures: &[],
        related_formats: &[],
    },
};
