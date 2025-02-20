use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
