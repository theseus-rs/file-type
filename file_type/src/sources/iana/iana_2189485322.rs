use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2189485322: FileType = FileType {
    file_format: &FileFormat {
        id: 2_189_485_322,
        source_type: SourceType::Iana,
        name: "vnd.octel.sbc",
        extensions: &[],
        media_types: &["audio/vnd.octel.sbc"],
        signatures: &[],
        related_formats: &[],
    },
};
