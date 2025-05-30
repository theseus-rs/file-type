use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2842844821: FileType = FileType {
    file_format: &FileFormat {
        id: 2_842_844_821,
        source_type: SourceType::Iana,
        name: "session-info",
        extensions: &[],
        media_types: &["application/session-info"],
        signatures: &[],
        related_formats: &[],
    },
};
