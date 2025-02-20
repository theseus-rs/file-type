use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3259703181: FileType = FileType {
    file_format: &FileFormat {
        id: 3_259_703_181,
        source_type: SourceType::Iana,
        name: "vnd.wasmflow.wafl",
        extensions: &[],
        media_types: &["application/vnd.wasmflow.wafl"],
        signatures: &[],
        related_formats: &[],
    },
};
