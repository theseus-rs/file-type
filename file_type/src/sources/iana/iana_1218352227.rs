use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1218352227: FileType = FileType {
    file_format: &FileFormat {
        id: 1_218_352_227,
        source_type: SourceType::Iana,
        name: "vnd.realvnc.bed",
        extensions: &[],
        media_types: &["application/vnd.realvnc.bed"],
        signatures: &[],
        related_formats: &[],
    },
};
