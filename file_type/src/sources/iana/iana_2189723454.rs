use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2189723454: FileType = FileType {
    file_format: &FileFormat {
        id: 2_189_723_454,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-floor-request+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-floor-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
