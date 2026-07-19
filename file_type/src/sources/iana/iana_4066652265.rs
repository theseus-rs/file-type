use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4066652265: FileType = FileType {
    file_format: &FileFormat {
        id: 4_066_652_265,
        source_type: SourceType::Iana,
        name: "vnd.tps",
        extensions: &[],
        media_types: &["text/vnd.tps"],
        signatures: &[],
        related_formats: &[],
    },
};
