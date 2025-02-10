use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1329015423: FileType = FileType {
    file_format: &FileFormat {
        id: 1_329_015_423,
        source_type: SourceType::Iana,
        name: "ipfix",
        extensions: &[],
        media_types: &["application/ipfix"],
        signatures: &[],
        related_formats: &[],
    },
};
