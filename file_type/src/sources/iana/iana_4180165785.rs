use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4180165785: FileType = FileType {
    file_format: &FileFormat {
        id: 4_180_165_785,
        source_type: SourceType::Iana,
        name: "mobile-xmf",
        extensions: &[],
        media_types: &["audio/mobile-xmf"],
        signatures: &[],
        related_formats: &[],
    },
};
