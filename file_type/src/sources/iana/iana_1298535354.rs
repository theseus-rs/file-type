use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1298535354: FileType = FileType {
    file_format: &FileFormat {
        id: 1_298_535_354,
        source_type: SourceType::Iana,
        name: "vnd.jcp.javame.midlet-rms",
        extensions: &[],
        media_types: &["application/vnd.jcp.javame.midlet-rms"],
        signatures: &[],
        related_formats: &[],
    },
};
