use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3896998504: FileType = FileType {
    file_format: &FileFormat {
        id: 3_896_998_504,
        source_type: SourceType::Iana,
        name: "encaprtp",
        extensions: &[],
        media_types: &["audio/encaprtp"],
        signatures: &[],
        related_formats: &[],
    },
};
