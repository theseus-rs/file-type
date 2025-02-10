use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_826043110: FileType = FileType {
    file_format: &FileFormat {
        id: 826_043_110,
        source_type: SourceType::Iana,
        name: "LPC",
        extensions: &[],
        media_types: &["audio/LPC"],
        signatures: &[],
        related_formats: &[],
    },
};
