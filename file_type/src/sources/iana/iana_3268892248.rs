use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3268892248: FileType = FileType {
    file_format: &FileFormat {
        id: 3_268_892_248,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mid-call+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mid-call+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
