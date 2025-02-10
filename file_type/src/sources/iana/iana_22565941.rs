use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_22565941: FileType = FileType {
    file_format: &FileFormat {
        id: 22_565_941,
        source_type: SourceType::Iana,
        name: "vnd.oma.scidm.messages+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.scidm.messages+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
