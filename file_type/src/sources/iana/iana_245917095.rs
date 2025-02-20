use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_245917095: FileType = FileType {
    file_format: &FileFormat {
        id: 245_917_095,
        source_type: SourceType::Iana,
        name: "vnd.rainstor.data",
        extensions: &[],
        media_types: &["application/vnd.rainstor.data"],
        signatures: &[],
        related_formats: &[],
    },
};
